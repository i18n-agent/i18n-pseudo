use clap::{Parser, ValueEnum};

/// Pseudo-translate i18n files for testing internationalization implementations.
///
/// Reuses i18n-convert's format parsers (32 formats) via auto-detection.
/// Applies configurable transformation strategies to translation values
/// while preserving file structure, keys, and placeholder tokens.
#[derive(Parser, Debug)]
#[command(name = "i18n-pseudo", version, about)]
pub struct Cli {
    /// Input file paths (format auto-detected via i18n-convert)
    #[arg(required = true)]
    pub files: Vec<String>,

    /// Output directory (default: stdout for single file)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Override format detection (e.g. json, i18next, android-xml)
    #[arg(short, long)]
    pub format: Option<String>,

    /// Modify files in-place (creates .bak backups)
    #[arg(long)]
    pub in_place: bool,

    /// Skip .bak backup when using --in-place
    #[arg(long)]
    pub no_backup: bool,

    /// Preset strategy combination
    #[arg(long, value_enum)]
    pub preset: Option<Preset>,

    // --- Individual strategy enable flags ---
    /// Enable accented characters
    #[arg(long)]
    pub accents: bool,

    /// Enable CJK character insertion
    #[arg(long)]
    pub cjk: bool,

    /// Enable special character insertion
    #[arg(long)]
    pub special_chars: bool,

    /// Enable text expansion (ratio 1.0-3.0)
    #[arg(long)]
    pub expansion: Option<f64>,

    /// Enable bracket wrapping
    #[arg(long)]
    pub brackets: bool,

    /// Enable RTL bidi markers
    #[arg(long)]
    pub rtl: bool,

    /// Enable unicode stress testing
    #[arg(long)]
    pub unicode_stress: bool,

    // --- Individual strategy disable flags (override preset) ---
    /// Disable accented characters (override preset)
    #[arg(long)]
    pub no_accents: bool,

    /// Disable CJK insertion (override preset)
    #[arg(long)]
    pub no_cjk: bool,

    /// Disable special character insertion (override preset)
    #[arg(long)]
    pub no_special_chars: bool,

    /// Disable text expansion (override preset)
    #[arg(long)]
    pub no_expansion: bool,

    /// Disable bracket wrapping (override preset)
    #[arg(long)]
    pub no_brackets: bool,

    /// Disable RTL bidi markers (override preset)
    #[arg(long)]
    pub no_rtl: bool,

    /// Disable unicode stress testing (override preset)
    #[arg(long)]
    pub no_unicode_stress: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Preset {
    /// accents + brackets — quick visual scan
    Default,
    /// expansion 1.5 + brackets — test UI overflow
    Layout,
    /// accents + cjk + special-chars + unicode-stress — encoding test
    Charset,
    /// rtl + brackets + expansion 1.3 — RTL layout testing
    Rtl,
    /// all 7 strategies, expansion 1.3 — kitchen sink
    Full,
}

/// Resolved strategy configuration after merging preset + individual flags.
#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub accents: bool,
    pub cjk: bool,
    pub special_chars: bool,
    pub expansion: Option<f64>,
    pub brackets: bool,
    pub rtl: bool,
    pub unicode_stress: bool,
}

impl StrategyConfig {
    /// Resolve strategy config from CLI args: start with preset defaults,
    /// then apply individual enable/disable overrides.
    pub fn from_cli(cli: &Cli) -> Self {
        // Start from preset defaults
        let (
            mut accents,
            mut cjk,
            mut special_chars,
            mut expansion,
            mut brackets,
            mut rtl,
            mut unicode_stress,
        ) = match cli.preset {
            Some(Preset::Default) | None => {
                if cli.preset.is_some() {
                    // Explicit --preset default
                    (true, false, false, None, true, false, false)
                } else {
                    // No preset: nothing enabled by default
                    (false, false, false, None, false, false, false)
                }
            }
            Some(Preset::Layout) => (false, false, false, Some(1.5), true, false, false),
            Some(Preset::Charset) => (true, true, true, None, false, false, true),
            Some(Preset::Rtl) => (false, false, false, Some(1.3), true, true, false),
            Some(Preset::Full) => (true, true, true, Some(1.3), true, true, true),
        };

        // Individual enable flags override (turn ON)
        if cli.accents {
            accents = true;
        }
        if cli.cjk {
            cjk = true;
        }
        if cli.special_chars {
            special_chars = true;
        }
        if cli.expansion.is_some() {
            expansion = cli.expansion;
        }
        if cli.brackets {
            brackets = true;
        }
        if cli.rtl {
            rtl = true;
        }
        if cli.unicode_stress {
            unicode_stress = true;
        }

        // Individual disable flags override (turn OFF)
        if cli.no_accents {
            accents = false;
        }
        if cli.no_cjk {
            cjk = false;
        }
        if cli.no_special_chars {
            special_chars = false;
        }
        if cli.no_expansion {
            expansion = None;
        }
        if cli.no_brackets {
            brackets = false;
        }
        if cli.no_rtl {
            rtl = false;
        }
        if cli.no_unicode_stress {
            unicode_stress = false;
        }

        // If no preset and no individual flags, fall back to default preset behavior
        let any_enabled = accents
            || cjk
            || special_chars
            || expansion.is_some()
            || brackets
            || rtl
            || unicode_stress;

        if !any_enabled && cli.preset.is_none() {
            accents = true;
            brackets = true;
        }

        StrategyConfig {
            accents,
            cjk,
            special_chars,
            expansion,
            brackets,
            rtl,
            unicode_stress,
        }
    }
}

/// Parse CLI arguments.
pub fn parse() -> Cli {
    Cli::parse()
}
