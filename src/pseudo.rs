use std::collections::HashMap;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::Path;
use std::sync::OnceLock;

use i18n_convert::detect::{detect_best, detect_format};
use i18n_convert::formats::FormatRegistry;
use i18n_convert::ir::{EntryValue, I18nResource, PluralSet};
use regex::Regex;
use thiserror::Error;

use crate::cli::{Cli, StrategyConfig};
use crate::strategies::{self, StrategyPipeline};

#[derive(Error, Debug)]
pub enum PseudoError {
    #[error("Ambiguous format for {path}: multiple candidates with equal confidence: {candidates}. Use -f to specify.")]
    AmbiguousFormat { path: String, candidates: String },

    #[error("No format detected for {path}. Use -f to specify.")]
    NoFormat { path: String },

    #[error("Unknown format: {0}")]
    UnknownFormat(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Write error: {0}")]
    Write(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid arguments: {0}")]
    InvalidArgs(String),
}

/// Main entry point: process all files according to CLI args.
pub fn run(cli: &Cli) -> Result<(), PseudoError> {
    let config = StrategyConfig::from_cli(cli);
    let pipeline = strategies::build_pipeline(&config);
    let registry = FormatRegistry::new();

    // Validate multi-file requires -o or --in-place
    if cli.files.len() > 1 && cli.output.is_none() && !cli.in_place {
        return Err(PseudoError::InvalidArgs(
            "Multiple files require -o <directory> or --in-place".to_string(),
        ));
    }

    // Validate expansion ratio if provided
    if let Some(ratio) = config.expansion {
        if !(1.0..=3.0).contains(&ratio) {
            return Err(PseudoError::InvalidArgs(format!(
                "Expansion ratio must be between 1.0 and 3.0, got {ratio}"
            )));
        }
    }

    for file_path in &cli.files {
        process_file(file_path, cli, &pipeline, &registry)?;
    }

    Ok(())
}

/// Process a single file: detect format, parse, transform, write.
fn process_file(
    file_path: &str,
    cli: &Cli,
    pipeline: &StrategyPipeline,
    registry: &FormatRegistry,
) -> Result<(), PseudoError> {
    let path = Path::new(file_path);
    let content = fs::read(path)?;

    // Determine format
    let format_id = if let Some(ref fmt) = cli.format {
        // User-specified format: verify it exists
        if registry.get(fmt).is_none() {
            return Err(PseudoError::UnknownFormat(fmt.clone()));
        }
        fmt.clone()
    } else {
        // Auto-detect
        resolve_format(registry, path, &content)?
    };

    let entry = registry
        .get(&format_id)
        .ok_or_else(|| PseudoError::UnknownFormat(format_id.clone()))?;

    // Parse
    let mut resource = entry
        .parser
        .parse(&content)
        .map_err(|e| PseudoError::Parse(e.to_string()))?;

    // Transform entries
    transform_resource(&mut resource, pipeline);

    // Write
    let output_bytes = entry
        .writer
        .write(&resource)
        .map_err(|e| PseudoError::Write(e.to_string()))?;

    // Output
    write_output(file_path, &output_bytes, cli)?;

    Ok(())
}

/// Resolve format from auto-detection, erroring on ambiguity.
fn resolve_format(
    registry: &FormatRegistry,
    path: &Path,
    content: &[u8],
) -> Result<String, PseudoError> {
    let results = detect_format(registry, path, content);

    if results.is_empty() {
        return Err(PseudoError::NoFormat {
            path: path.display().to_string(),
        });
    }

    // Check for ambiguity: multiple candidates with the same top confidence
    let top_confidence = results[0].1;
    let top_candidates: Vec<&str> = results
        .iter()
        .filter(|(_, c)| *c == top_confidence)
        .map(|(id, _)| *id)
        .collect();

    if top_candidates.len() > 1 {
        return Err(PseudoError::AmbiguousFormat {
            path: path.display().to_string(),
            candidates: top_candidates.join(", "),
        });
    }

    // Use detect_best for the single best match
    detect_best(registry, path, content)
        .map(|s| s.to_string())
        .ok_or_else(|| PseudoError::NoFormat {
            path: path.display().to_string(),
        })
}

/// Write output to the appropriate destination.
fn write_output(file_path: &str, output: &[u8], cli: &Cli) -> Result<(), PseudoError> {
    if cli.in_place {
        // In-place: create backup (unless --no-backup), then overwrite
        if !cli.no_backup {
            let backup_path = format!("{file_path}.bak");
            fs::copy(file_path, &backup_path)?;
        }
        fs::write(file_path, output)?;
    } else if let Some(ref output_dir) = cli.output {
        // Write to output directory
        let out_path = Path::new(output_dir);
        fs::create_dir_all(out_path)?;
        let filename = Path::new(file_path).file_name().ok_or_else(|| {
            PseudoError::InvalidArgs(format!("Cannot extract filename from {file_path}"))
        })?;
        let dest = out_path.join(filename);
        fs::write(dest, output)?;
    } else {
        // Single file to stdout
        io::stdout().write_all(output)?;
    }

    Ok(())
}

/// Transform all entries in a resource using the strategy pipeline.
fn transform_resource(resource: &mut I18nResource, pipeline: &StrategyPipeline) {
    for entry in resource.entries.values_mut() {
        if entry.translatable == Some(false) {
            continue;
        }
        let placeholders = &entry.placeholders;
        transform_entry_value(&mut entry.value, placeholders, pipeline);
    }
}

/// Transform an EntryValue, recursively visiting all string values.
fn transform_entry_value(
    value: &mut EntryValue,
    placeholders: &[i18n_convert::ir::Placeholder],
    pipeline: &StrategyPipeline,
) {
    match value {
        EntryValue::Simple(ref mut text) => {
            *text = transform_text(text, placeholders, pipeline);
        }
        EntryValue::Plural(ref mut plural) => {
            transform_plural_set(plural, placeholders, pipeline);
        }
        EntryValue::Array(ref mut items) => {
            for item in items.iter_mut() {
                *item = transform_text(item, placeholders, pipeline);
            }
        }
        EntryValue::Select(ref mut select) => {
            for case_value in select.cases.values_mut() {
                *case_value = transform_text(case_value, placeholders, pipeline);
            }
        }
        EntryValue::MultiVariablePlural(ref mut mvp) => {
            mvp.pattern = transform_text(&mvp.pattern, placeholders, pipeline);
            for var in mvp.variables.values_mut() {
                transform_plural_set(&mut var.plural_set, placeholders, pipeline);
            }
        }
    }
}

/// Transform all forms in a PluralSet.
fn transform_plural_set(
    plural: &mut PluralSet,
    placeholders: &[i18n_convert::ir::Placeholder],
    pipeline: &StrategyPipeline,
) {
    if let Some(ref mut z) = plural.zero {
        *z = transform_text(z, placeholders, pipeline);
    }
    if let Some(ref mut o) = plural.one {
        *o = transform_text(o, placeholders, pipeline);
    }
    if let Some(ref mut t) = plural.two {
        *t = transform_text(t, placeholders, pipeline);
    }
    if let Some(ref mut f) = plural.few {
        *f = transform_text(f, placeholders, pipeline);
    }
    if let Some(ref mut m) = plural.many {
        *m = transform_text(m, placeholders, pipeline);
    }
    plural.other = transform_text(&plural.other, placeholders, pipeline);

    for val in plural.exact_matches.values_mut() {
        *val = transform_text(val, placeholders, pipeline);
    }
    for range in plural.range_matches.iter_mut() {
        range.value = transform_text(&range.value, placeholders, pipeline);
    }
}

/// Build an opaque single-character token from the Unicode Private Use Area.
/// Each token is a single PUA character (U+E000 + counter), making it
/// impossible for any strategy to corrupt by inserting characters in the middle.
/// Supports up to 6400 tokens per text (U+E000..U+F8FF).
fn make_token(counter: u32) -> String {
    let c = char::from_u32(0xE000 + counter).expect("token counter exceeded PUA range");
    c.to_string()
}

/// Transform a single text string, preserving placeholders and HTML tags.
///
/// 1. Replace placeholders and HTML tags with opaque tokens
/// 2. Run the strategy pipeline on the cleaned text
/// 3. Restore the original placeholders and HTML tags
fn transform_text(
    text: &str,
    placeholders: &[i18n_convert::ir::Placeholder],
    pipeline: &StrategyPipeline,
) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut working = text.to_string();
    let mut token_map: HashMap<String, String> = HashMap::new();
    let mut counter = 0u32;

    // Replace IR-parsed placeholders with tokens
    for ph in placeholders {
        if ph.original_syntax.is_empty() {
            continue;
        }
        // Only replace if the placeholder actually appears in this text
        if working.contains(&ph.original_syntax) {
            let token = make_token(counter);
            counter += 1;
            // Replace all occurrences of this placeholder
            working = working.replace(&ph.original_syntax, &token);
            token_map.insert(token, ph.original_syntax.clone());
        }
    }

    // Replace HTML tags with tokens (not tracked in IR placeholder data)
    static HTML_RE: OnceLock<Regex> = OnceLock::new();
    let html_re = HTML_RE.get_or_init(|| {
        Regex::new(r"</?[a-zA-Z][a-zA-Z0-9]*(?:\s+[^>]*)?>")
            .expect("HTML regex pattern is a valid constant")
    });
    let html_matches: Vec<String> = html_re
        .find_iter(&working)
        .map(|m| m.as_str().to_string())
        .collect();
    for tag in &html_matches {
        if working.contains(tag.as_str()) {
            let token = make_token(counter);
            counter += 1;
            working = working.replacen(tag.as_str(), &token, 1);
            token_map.insert(token, tag.clone());
        }
    }

    // Apply the strategy pipeline
    working = pipeline.apply(&working);

    // Restore tokens
    for (token, original) in &token_map {
        working = working.replace(token, original);
    }

    working
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::StrategyConfig;
    use crate::strategies;
    use i18n_convert::ir::{I18nEntry, Placeholder};

    fn default_pipeline() -> StrategyPipeline {
        let config = StrategyConfig {
            accents: true,
            cjk: false,
            special_chars: false,
            expansion: None,
            brackets: true,
            rtl: false,
            unicode_stress: false,
        };
        strategies::build_pipeline(&config)
    }

    #[test]
    fn test_transform_text_empty() {
        let pipeline = default_pipeline();
        let result = transform_text("", &[], &pipeline);
        assert_eq!(result, "");
    }

    #[test]
    fn test_transform_text_preserves_placeholders() {
        let pipeline = default_pipeline();
        let placeholders = vec![Placeholder {
            name: "name".to_string(),
            original_syntax: "{name}".to_string(),
            placeholder_type: None,
            position: None,
            example: None,
            description: None,
            format: None,
            optional_parameters: None,
        }];
        let result = transform_text("Hello {name}!", &placeholders, &pipeline);
        assert!(
            result.contains("{name}"),
            "Placeholder should be preserved, got: {result}"
        );
    }

    #[test]
    fn test_transform_text_preserves_html() {
        let pipeline = default_pipeline();
        let result = transform_text("Click <b>here</b> now", &[], &pipeline);
        assert!(
            result.contains("<b>") && result.contains("</b>"),
            "HTML tags should be preserved, got: {result}"
        );
    }

    #[test]
    fn test_translatable_false_skipped() {
        let pipeline = default_pipeline();
        let mut resource = I18nResource {
            metadata: i18n_convert::ir::ResourceMetadata::default(),
            entries: indexmap::IndexMap::new(),
        };
        resource.entries.insert(
            "key1".to_string(),
            I18nEntry {
                key: "key1".to_string(),
                value: EntryValue::Simple("Do not touch".to_string()),
                translatable: Some(false),
                ..I18nEntry::default()
            },
        );
        resource.entries.insert(
            "key2".to_string(),
            I18nEntry {
                key: "key2".to_string(),
                value: EntryValue::Simple("Hello".to_string()),
                translatable: None,
                ..I18nEntry::default()
            },
        );

        transform_resource(&mut resource, &pipeline);

        assert_eq!(
            resource.entries["key1"].value,
            EntryValue::Simple("Do not touch".to_string()),
            "translatable=false entry should be unchanged"
        );
        // key2 should be transformed
        if let EntryValue::Simple(ref val) = resource.entries["key2"].value {
            assert_ne!(
                val, "Hello",
                "translatable=None entry should be transformed"
            );
        }
    }
}
