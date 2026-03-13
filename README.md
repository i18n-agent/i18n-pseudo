# i18n-pseudo

[![CI](https://github.com/i18n-agent/i18n-pseudo/actions/workflows/ci.yml/badge.svg)](https://github.com/i18n-agent/i18n-pseudo/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/i18n-agent/i18n-pseudo)](LICENSE)

Pseudo-translate i18n localization files for testing internationalization implementations. Supports 32 formats via [i18n-convert](https://github.com/i18n-agent/i18n-convert).

## The Problem

Testing internationalization is tedious. Real translations are expensive and slow to produce. Without translated content, you can't tell if your UI handles text expansion, RTL scripts, special characters, or missing translations correctly.

## The Solution

One binary. Feed it any localization file, get back a pseudo-translated version that exposes i18n issues instantly --- while preserving placeholders, ICU syntax, and file structure.

```bash
i18n-pseudo en.json -o pseudo.json
```

## Features

- **7 pseudo-translation strategies** --- accents, CJK, special characters, expansion, brackets, RTL, and unicode stress testing
- **5 presets** --- quick shorthand for common testing scenarios
- **32 format support** --- every format supported by [i18n-convert](https://github.com/i18n-agent/i18n-convert)
- **Placeholder preservation** --- `{name}`, `{{count}}`, `%s`, `%d`, `{0}`, ICU expressions are never mangled
- **Pipe-friendly** --- reads from stdin, writes to stdout by default

## Installation

### npm (recommended)

```bash
npm install -g @i18n-agent/i18n-pseudo
```

### Homebrew

```bash
brew tap i18n-agent/tap
brew install i18n-pseudo
```

### Download binary

Download the latest release for your platform from [GitHub Releases](https://github.com/i18n-agent/i18n-pseudo/releases/latest).

**macOS (Apple Silicon):**
```bash
curl -L https://github.com/i18n-agent/i18n-pseudo/releases/latest/download/i18n-pseudo-aarch64-apple-darwin.tar.gz | tar xz
sudo mv i18n-pseudo /usr/local/bin/
```

**macOS (Intel):**
```bash
curl -L https://github.com/i18n-agent/i18n-pseudo/releases/latest/download/i18n-pseudo-x86_64-apple-darwin.tar.gz | tar xz
sudo mv i18n-pseudo /usr/local/bin/
```

**Linux (x64):**
```bash
curl -L https://github.com/i18n-agent/i18n-pseudo/releases/latest/download/i18n-pseudo-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv i18n-pseudo /usr/local/bin/
```

**Linux (ARM):**
```bash
curl -L https://github.com/i18n-agent/i18n-pseudo/releases/latest/download/i18n-pseudo-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv i18n-pseudo /usr/local/bin/
```

**Windows:**

Download `i18n-pseudo-x86_64-pc-windows-msvc.zip` from [Releases](https://github.com/i18n-agent/i18n-pseudo/releases/latest), extract, and add to your PATH.

### Build from source

```bash
git clone https://github.com/i18n-agent/i18n-pseudo.git
cd i18n-pseudo
cargo build --release
# Binary at target/release/i18n-pseudo
```

## Quick Start

```bash
# Pseudo-translate a JSON file with default settings (accents strategy)
i18n-pseudo en.json -o pseudo.json

# Use a preset for UI testing
i18n-pseudo en.json --preset ui-review -o pseudo.json

# Apply multiple strategies
i18n-pseudo en.json --strategy accents --strategy expansion -o pseudo.json

# Pseudo-translate Android XML
i18n-pseudo strings.xml -o strings-pseudo.xml

# Pipe from stdin
cat en.json | i18n-pseudo --format json --strategy brackets
```

## Strategies

Each strategy transforms translation values in a different way to expose specific i18n issues.

| Strategy | Description | Example Input | Example Output |
|----------|-------------|---------------|----------------|
| `accents` | Replaces ASCII with accented equivalents | `Hello World` | `Helloo Woorld` |
| `cjk` | Replaces characters with CJK equivalents | `Hello` | `太尔尔口` |
| `special-chars` | Inserts special/diacritic characters | `Hello` | `H!e@l#l$o%` |
| `expansion` | Pads text to simulate language expansion (~30%) | `Save` | `Save Lorem ip` |
| `brackets` | Wraps values in brackets for visual detection | `Hello` | `[Hello]` |
| `rtl` | Wraps text with RTL marks for bidi testing | `Hello` | (RTL-wrapped text) |
| `unicode-stress` | Inserts combining marks, zero-width chars | `Hello` | `H\u0300e\u0301l\u0302l\u0303o` |

## Presets

Presets are shorthand for common strategy combinations.

| Preset | Strategies Enabled | Use Case |
|--------|--------------------|----------|
| `quick` | `accents` | Fast visual check that strings are externalized |
| `ui-review` | `accents` + `expansion` + `brackets` | Full UI review --- see expansion issues and unlocalized strings |
| `bidi` | `rtl` + `brackets` | RTL / bidirectional text testing |
| `stress` | `unicode-stress` + `expansion` + `special-chars` | Push rendering to the limit |
| `cjk-check` | `cjk` + `expansion` | Test CJK character rendering and text expansion |

## CLI Reference

```
Usage: i18n-pseudo [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file (reads from stdin if omitted)

Options:
  -o, --output <FILE>          Output file (default: stdout)
      --format <FORMAT>        Input format (auto-detected if omitted)
  -s, --strategy <STRATEGY>    Strategy to apply (can be repeated)
  -p, --preset <PRESET>        Use a named preset
      --expansion-ratio <N>    Text expansion ratio (default: 0.3)
      --verbose                Show processing details
  -V, --version                Print version
  -h, --help                   Print help
```

When no `--strategy` or `--preset` is specified, the `accents` strategy is used by default.

## Supported Formats (32)

All formats supported by [i18n-convert](https://github.com/i18n-agent/i18n-convert) work out of the box.

### Mobile & Desktop

| Format | ID | Extensions |
|--------|----|------------|
| Android XML | `android-xml` | `.xml` |
| Xcode String Catalog | `xcstrings` | `.xcstrings` |
| iOS Strings | `ios-strings` | `.strings` |
| iOS Stringsdict | `stringsdict` | `.stringsdict` |
| iOS Property List | `ios-plist` | `.plist` |
| Flutter ARB | `arb` | `.arb` |
| Qt Linguist | `qt` | `.ts` |

### Web & Frameworks

| Format | ID | Extensions |
|--------|----|------------|
| Structured JSON | `json` | `.json` |
| i18next JSON | `i18next` | `.json` |
| JSON5 | `json5` | `.json5` |
| HJSON | `hjson` | `.hjson` |
| YAML (Rails) | `yaml-rails` | `.yml` `.yaml` |
| YAML (Plain) | `yaml-plain` | `.yml` `.yaml` |
| JavaScript | `javascript` | `.js` |
| TypeScript | `typescript` | `.ts` |
| PHP/Laravel | `php-laravel` | `.php` |
| NEON | `neon` | `.neon` |

### Standards & Exchange

| Format | ID | Extensions |
|--------|----|------------|
| XLIFF 1.2 | `xliff` | `.xliff` `.xlf` |
| XLIFF 2.0 | `xliff2` | `.xliff` `.xlf` |
| Gettext PO | `po` | `.po` `.pot` |
| TMX | `tmx` | `.tmx` |
| .NET RESX | `resx` | `.resx` |
| Java Properties | `java-properties` | `.properties` |

### Data & Other

| Format | ID | Extensions |
|--------|----|------------|
| CSV | `csv` | `.csv` `.tsv` |
| Excel | `excel` | `.xlsx` `.xls` |
| TOML | `toml` | `.toml` |
| INI | `ini` | `.ini` |
| SRT Subtitles | `srt` | `.srt` |
| Markdown | `markdown` | `.md` |
| Plain Text | `plain-text` | `.txt` |

### Vendor-Specific

| Format | ID | Extensions |
|--------|----|------------|
| iSpring Suite XLIFF | `ispring-xliff` | `.xliff` `.xlf` |
| Adobe Captivate XML | `captivate-xml` | `.xml` |

## Examples

### JSON

**Input (`en.json`):**
```json
{
  "greeting": "Hello, {name}!",
  "items": "{count, plural, one {# item} other {# items}}"
}
```

**Output (`pseudo.json` with `--preset ui-review`):**
```json
{
  "greeting": "[Helloo, {name}! Lorem ipsum]",
  "items": "{count, plural, one {[# iiTem Lorem]} other {[# iiTems Lorem ip]}}"
}
```

### Android XML

**Input (`strings.xml`):**
```xml
<resources>
    <string name="app_name">My App</string>
    <string name="welcome">Welcome, %s!</string>
</resources>
```

**Output (with `--strategy accents`):**
```xml
<resources>
    <string name="app_name">My Aapp</string>
    <string name="welcome">Weelcoomee, %s!</string>
</resources>
```

### iOS Strings

**Input (`Localizable.strings`):**
```
"save_button" = "Save";
"greeting" = "Hello, %@!";
```

**Output (with `--strategy brackets`):**
```
"save_button" = "[Save]";
"greeting" = "[Hello, %@!]";
```

## Contributing

- Open issues for bugs or feature requests.
- PRs welcome.
- Run `cargo test` before submitting.

## License

MIT License - see [LICENSE](LICENSE) file.

Built by [i18nagent.ai](https://i18nagent.ai)
