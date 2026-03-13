use i18n_convert::formats::FormatRegistry;
use i18n_convert::ir::*;
use i18n_pseudo::cli::StrategyConfig;
use i18n_pseudo::strategies::{self, StrategyPipeline};
use indexmap::IndexMap;

fn make_config(accents: bool, brackets: bool) -> StrategyConfig {
    StrategyConfig {
        accents,
        cjk: false,
        special_chars: false,
        expansion: None,
        brackets,
        rtl: false,
        unicode_stress: false,
    }
}

fn make_full_config() -> StrategyConfig {
    StrategyConfig {
        accents: true,
        cjk: true,
        special_chars: true,
        expansion: Some(1.3),
        brackets: true,
        rtl: true,
        unicode_stress: true,
    }
}

// ─── JSON round-trip ────────────────────────────────────────────────────────

#[test]
fn json_round_trip_structure_preserved() {
    let registry = FormatRegistry::new();
    let json_entry = registry.get("json").expect("json format should exist");

    // Create a simple JSON i18n file
    let json_content = br#"{"greeting": "Hello", "farewell": "Goodbye"}"#;

    let mut resource = json_entry
        .parser
        .parse(json_content)
        .expect("should parse JSON");

    // Apply pseudo-translation
    let pipeline = strategies::build_pipeline(&make_config(true, true));
    for entry in resource.entries.values_mut() {
        if entry.translatable == Some(false) {
            continue;
        }
        transform_value(&mut entry.value, &entry.placeholders, &pipeline);
    }

    // Write back
    let output = json_entry
        .writer
        .write(&resource)
        .expect("should write JSON");
    let output_str = String::from_utf8(output).expect("valid UTF-8");

    // Structure should be valid JSON with our keys
    assert!(output_str.contains("greeting"), "Key 'greeting' preserved");
    assert!(output_str.contains("farewell"), "Key 'farewell' preserved");

    // Values should be transformed (contain brackets from bracket strategy)
    assert!(
        output_str.contains('['),
        "Values should be bracket-wrapped: {output_str}"
    );
}

#[test]
fn placeholder_preservation_through_pipeline() {
    let pipeline = strategies::build_pipeline(&make_full_config());

    let placeholders = vec![
        Placeholder {
            name: "name".to_string(),
            original_syntax: "{name}".to_string(),
            placeholder_type: None,
            position: None,
            example: None,
            description: None,
            format: None,
            optional_parameters: None,
        },
        Placeholder {
            name: "count".to_string(),
            original_syntax: "{count}".to_string(),
            placeholder_type: None,
            position: None,
            example: None,
            description: None,
            format: None,
            optional_parameters: None,
        },
    ];

    let text = "Hello {name}, you have {count} items";
    let result = transform_text_with_placeholders(text, &placeholders, &pipeline);

    assert!(
        result.contains("{name}"),
        "Placeholder {{name}} should be preserved: {result}"
    );
    assert!(
        result.contains("{count}"),
        "Placeholder {{count}} should be preserved: {result}"
    );
}

#[test]
fn translatable_false_entries_skipped() {
    let pipeline = strategies::build_pipeline(&make_config(true, true));

    let mut resource = I18nResource {
        metadata: ResourceMetadata::default(),
        entries: IndexMap::new(),
    };

    // Entry that should NOT be transformed
    resource.entries.insert(
        "app_name".to_string(),
        I18nEntry {
            key: "app_name".to_string(),
            value: EntryValue::Simple("MyApp".to_string()),
            translatable: Some(false),
            ..I18nEntry::default()
        },
    );

    // Entry that SHOULD be transformed
    resource.entries.insert(
        "greeting".to_string(),
        I18nEntry {
            key: "greeting".to_string(),
            value: EntryValue::Simple("Hello".to_string()),
            translatable: None,
            ..I18nEntry::default()
        },
    );

    for entry in resource.entries.values_mut() {
        if entry.translatable == Some(false) {
            continue;
        }
        transform_value(&mut entry.value, &entry.placeholders, &pipeline);
    }

    // app_name should be unchanged
    if let EntryValue::Simple(ref val) = resource.entries["app_name"].value {
        assert_eq!(val, "MyApp", "translatable=false should be unchanged");
    }

    // greeting should be transformed
    if let EntryValue::Simple(ref val) = resource.entries["greeting"].value {
        assert_ne!(val, "Hello", "translatable=None should be transformed");
        assert!(val.contains('['), "Should have brackets: {val}");
    }
}

#[test]
fn preset_default_uses_accents_and_brackets() {
    let config = make_config(true, true);
    let pipeline = strategies::build_pipeline(&config);

    assert_eq!(pipeline.len(), 2, "default preset = accents + brackets");

    let result = pipeline.apply("Hello");
    assert!(
        result.starts_with('['),
        "Should start with bracket: {result}"
    );
    assert!(result.ends_with(']'), "Should end with bracket: {result}");
    // Inside brackets, chars should be accented
    assert!(
        result.contains('\u{00E9}'),
        "Should contain accented 'e': {result}"
    );
}

#[test]
fn plural_values_all_transformed() {
    let pipeline = strategies::build_pipeline(&make_config(true, true));

    let mut value = EntryValue::Plural(PluralSet {
        zero: Some("no items".to_string()),
        one: Some("one item".to_string()),
        two: None,
        few: None,
        many: None,
        other: "many items".to_string(),
        exact_matches: IndexMap::new(),
        range_matches: Vec::new(),
        ordinal: false,
    });

    transform_value(&mut value, &[], &pipeline);

    if let EntryValue::Plural(ref plural) = value {
        assert!(
            plural.zero.as_ref().unwrap().contains('['),
            "zero form should be transformed"
        );
        assert!(
            plural.one.as_ref().unwrap().contains('['),
            "one form should be transformed"
        );
        assert!(
            plural.other.contains('['),
            "other form should be transformed"
        );
    } else {
        panic!("Expected Plural variant");
    }
}

#[test]
fn array_values_all_transformed() {
    let pipeline = strategies::build_pipeline(&make_config(true, true));

    let mut value = EntryValue::Array(vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ]);

    transform_value(&mut value, &[], &pipeline);

    if let EntryValue::Array(ref items) = value {
        for (i, item) in items.iter().enumerate() {
            assert!(item.contains('['), "item {i} should be transformed: {item}");
        }
    }
}

#[test]
fn select_values_transformed() {
    let pipeline = strategies::build_pipeline(&make_config(true, true));

    let mut cases = IndexMap::new();
    cases.insert("male".to_string(), "He said hello".to_string());
    cases.insert("female".to_string(), "She said hello".to_string());

    let mut value = EntryValue::Select(SelectSet {
        variable: "gender".to_string(),
        cases,
    });

    transform_value(&mut value, &[], &pipeline);

    if let EntryValue::Select(ref select) = value {
        assert_eq!(
            select.variable, "gender",
            "variable name should be unchanged"
        );
        for (key, val) in &select.cases {
            assert!(
                val.contains('['),
                "case '{key}' value should be transformed: {val}"
            );
        }
    }
}

#[test]
fn html_tags_preserved_in_pipeline() {
    let pipeline = strategies::build_pipeline(&make_config(true, true));
    let result = transform_text_with_placeholders("Click <b>here</b> please", &[], &pipeline);
    assert!(
        result.contains("<b>") && result.contains("</b>"),
        "HTML tags should be preserved: {result}"
    );
}

#[test]
fn empty_pipeline_no_change() {
    let config = StrategyConfig {
        accents: false,
        cjk: false,
        special_chars: false,
        expansion: None,
        brackets: false,
        rtl: false,
        unicode_stress: false,
    };
    let pipeline = strategies::build_pipeline(&config);
    assert!(pipeline.is_empty());
    assert_eq!(pipeline.apply("Hello"), "Hello");
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Single PUA character token (same as pseudo.rs).
fn make_token(counter: u32) -> String {
    let c = char::from_u32(0xE000 + counter).expect("token counter exceeded PUA range");
    c.to_string()
}

/// Transform a single text using placeholder preservation (mirrors pseudo.rs logic).
fn transform_text_with_placeholders(
    text: &str,
    placeholders: &[Placeholder],
    pipeline: &StrategyPipeline,
) -> String {
    use std::collections::HashMap;

    if text.is_empty() {
        return String::new();
    }

    let mut working = text.to_string();
    let mut token_map: HashMap<String, String> = HashMap::new();
    let mut counter = 0u32;

    for ph in placeholders {
        if ph.original_syntax.is_empty() {
            continue;
        }
        if working.contains(&ph.original_syntax) {
            let token = make_token(counter);
            counter += 1;
            working = working.replace(&ph.original_syntax, &token);
            token_map.insert(token, ph.original_syntax.clone());
        }
    }

    let html_re = regex::Regex::new(r"</?[a-zA-Z][a-zA-Z0-9]*(?:\s+[^>]*)?>").unwrap();
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

    working = pipeline.apply(&working);

    for (token, original) in &token_map {
        working = working.replace(token, original);
    }

    working
}

/// Transform an EntryValue (mirrors pseudo.rs logic for testing).
fn transform_value(
    value: &mut EntryValue,
    placeholders: &[Placeholder],
    pipeline: &StrategyPipeline,
) {
    match value {
        EntryValue::Simple(ref mut text) => {
            *text = transform_text_with_placeholders(text, placeholders, pipeline);
        }
        EntryValue::Plural(ref mut plural) => {
            if let Some(ref mut z) = plural.zero {
                *z = transform_text_with_placeholders(z, placeholders, pipeline);
            }
            if let Some(ref mut o) = plural.one {
                *o = transform_text_with_placeholders(o, placeholders, pipeline);
            }
            if let Some(ref mut t) = plural.two {
                *t = transform_text_with_placeholders(t, placeholders, pipeline);
            }
            if let Some(ref mut f) = plural.few {
                *f = transform_text_with_placeholders(f, placeholders, pipeline);
            }
            if let Some(ref mut m) = plural.many {
                *m = transform_text_with_placeholders(m, placeholders, pipeline);
            }
            plural.other = transform_text_with_placeholders(&plural.other, placeholders, pipeline);
            for val in plural.exact_matches.values_mut() {
                *val = transform_text_with_placeholders(val, placeholders, pipeline);
            }
            for range in plural.range_matches.iter_mut() {
                range.value =
                    transform_text_with_placeholders(&range.value, placeholders, pipeline);
            }
        }
        EntryValue::Array(ref mut items) => {
            for item in items.iter_mut() {
                *item = transform_text_with_placeholders(item, placeholders, pipeline);
            }
        }
        EntryValue::Select(ref mut select) => {
            for case_value in select.cases.values_mut() {
                *case_value = transform_text_with_placeholders(case_value, placeholders, pipeline);
            }
        }
        EntryValue::MultiVariablePlural(ref mut mvp) => {
            mvp.pattern = transform_text_with_placeholders(&mvp.pattern, placeholders, pipeline);
            for var in mvp.variables.values_mut() {
                let ps = &mut var.plural_set;
                if let Some(ref mut z) = ps.zero {
                    *z = transform_text_with_placeholders(z, placeholders, pipeline);
                }
                if let Some(ref mut o) = ps.one {
                    *o = transform_text_with_placeholders(o, placeholders, pipeline);
                }
                if let Some(ref mut t) = ps.two {
                    *t = transform_text_with_placeholders(t, placeholders, pipeline);
                }
                if let Some(ref mut f) = ps.few {
                    *f = transform_text_with_placeholders(f, placeholders, pipeline);
                }
                if let Some(ref mut m) = ps.many {
                    *m = transform_text_with_placeholders(m, placeholders, pipeline);
                }
                ps.other = transform_text_with_placeholders(&ps.other, placeholders, pipeline);
            }
        }
    }
}
