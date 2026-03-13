# i18n-pseudo

[![CI](https://github.com/i18n-agent/i18n-pseudo/actions/workflows/ci.yml/badge.svg)](https://github.com/i18n-agent/i18n-pseudo/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/i18n-agent/i18n-pseudo)](LICENSE)

Pseudo-translate i18n localization files for testing internationalization implementations. Supports 32 formats via [i18n-convert](https://github.com/i18n-agent/i18n-convert).

## The Problem

Testing internationalization is tedious. Real translations are expensive and slow to produce. Without translated content, you can't tell if your UI handles text expansion, RTL scripts, special characters, or missing translations correctly.

## The Solution

One binary. Feed it any localization file, get back a pseudo-translated version that exposes i18n issues instantly — while preserving placeholders, file structure, and keys.

```bash
i18n-pseudo en.json -o output/
```

## Features

- **7 pseudo-translation strategies** — accents, CJK, special characters, expansion, brackets, RTL, and unicode stress testing
- **5 presets** — shorthand for common testing scenarios
- **32 format support** — every format supported by [i18n-convert](https://github.com/i18n-agent/i18n-convert)
- **Placeholder preservation** — `{name}`, `{{count}}`, `%s`, `%d`, `${var}`, HTML tags are never mangled
- **Multi-file input** — process multiple files in one command

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
# Pseudo-translate with default (accents + brackets)
i18n-pseudo en.json -o output/

# Use a preset for layout testing
i18n-pseudo en.json --preset layout -o output/

# Apply specific strategies
i18n-pseudo en.json --accents --expansion 1.5 -o output/

# Override preset: layout preset minus brackets
i18n-pseudo en.json --preset layout --no-brackets -o output/

# Multiple files at once
i18n-pseudo en.json strings.xml messages.po -o output/

# In-place modification (creates .bak backups)
i18n-pseudo en.json --in-place --accents

# Single file to stdout
i18n-pseudo en.json --brackets
```

## Strategies

Each strategy transforms translation values to expose specific i18n issues.

| Strategy | Flag | Example Input | Example Output |
|----------|------|---------------|----------------|
| Accents | `--accents` | `Hello World` | `Ĥéľľó Ẃóŕľð` |
| CJK | `--cjk` | `Hello World` | `Hello中 Worl韓d` |
| Special chars | `--special-chars` | `Hello World` | `Hello§ W¶orld` |
| Expansion | `--expansion <1.0-3.0>` | `Save` | `Save~~~` |
| Brackets | `--brackets` | `Hello` | `[Hello]` |
| RTL | `--rtl` | `Hello` | `\u202BHello\u202C` |
| Unicode stress | `--unicode-stress` | `Hello` | `H̀é̀l̂l̃o` |

## Presets

Presets are shorthand for common strategy combinations. Individual flags override preset defaults.

| Preset | Strategies | Use Case |
|--------|------------|----------|
| `default` | accents + brackets | Quick visual scan for untranslated strings |
| `layout` | expansion 1.5 + brackets | Test UI overflow/truncation |
| `charset` | accents + cjk + special-chars + unicode-stress | Character rendering/encoding |
| `rtl` | rtl + brackets + expansion 1.3 | Right-to-left layout testing |
| `full` | all 7, expansion 1.3 | Kitchen sink |

```bash
# Use full preset but disable CJK
i18n-pseudo en.json --preset full --no-cjk -o output/
```

## CLI Reference

```
USAGE:
    i18n-pseudo [OPTIONS] <FILES>...

ARGS:
    <FILES>...    Input file paths (format auto-detected via i18n-convert)

OPTIONS:
    -o, --output <PATH>         Output directory (default: stdout for single file)
    -f, --format <FORMAT>       Override format detection (e.g. json, android-xml)
        --in-place              Modify files in-place (creates .bak backups)
        --no-backup             Skip .bak when using --in-place

        --preset <PRESET>       default, layout, charset, rtl, full
        --accents               Enable accented characters
        --cjk                   Enable CJK insertion
        --special-chars         Enable special character insertion
        --expansion <RATIO>     Enable text expansion (1.0-3.0)
        --brackets              Enable bracket wrapping
        --rtl                   Enable RTL bidi markers
        --unicode-stress        Enable unicode stress testing
        --no-accents            Disable (override preset)
        --no-cjk                Disable (override preset)
        --no-special-chars      Disable (override preset)
        --no-expansion          Disable (override preset)
        --no-brackets           Disable (override preset)
        --no-rtl                Disable (override preset)
        --no-unicode-stress     Disable (override preset)

    -h, --help                  Print help
    -V, --version               Print version
```

When no `--preset` or individual flags are specified, defaults to accents + brackets.

### Multi-file Behavior

- **Single file, no `-o`**: prints to stdout
- **Single file + `-o dir/`**: writes to `dir/filename`
- **Multiple files**: `-o` required, must be a directory
- **`--in-place`**: overwrites originals with `.bak` backup

### Exit Codes

- `0` — success
- `1` — parse/write/IO error
- `2` — invalid arguments or ambiguous format detection

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
  "save": "Save"
}
```

**Output (default: accents + brackets):**
```json
{
  "greeting": "[Ĥéľľó, {name}!]",
  "save": "[Šáṽé]"
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

**Output (`--accents`):**
```xml
<resources>
    <string name="app_name">Ṁý Áþþ</string>
    <string name="welcome">Ẃéľçóɱé, %s!</string>
</resources>
```

### iOS Strings

**Input (`Localizable.strings`):**
```
"save_button" = "Save";
"greeting" = "Hello, %@!";
```

**Output (`--brackets`):**
```
"save_button" = "[Save]";
"greeting" = "[Hello, %@!]";
```

## Known Limitations

- **ICU message syntax** inside `Simple` values (e.g., `{count, plural, one {# item} other {# items}}`) may be corrupted. Well-parsed files use the format's native plural/select support which maps to the IR's `Plural`/`Select` variants and is handled correctly.
- **Ambiguous formats** (e.g., `.json` could be structured JSON or i18next) require `-f` to specify. The CLI will exit with code 2 listing candidates.

## Contributing

- Open issues for bugs or feature requests.
- PRs welcome.
- Run `cargo test` before submitting.

## License

MIT License — see [LICENSE](LICENSE) file.

Built by [i18nagent.ai](https://i18nagent.ai)
