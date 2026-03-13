use i18n_pseudo::strategies::accents::AccentStrategy;
use i18n_pseudo::strategies::brackets::BracketStrategy;
use i18n_pseudo::strategies::cjk::CjkStrategy;
use i18n_pseudo::strategies::expansion::ExpansionStrategy;
use i18n_pseudo::strategies::rtl::RtlStrategy;
use i18n_pseudo::strategies::special_chars::SpecialCharsStrategy;
use i18n_pseudo::strategies::unicode_stress::UnicodeStressStrategy;
use i18n_pseudo::strategies::Strategy;

// ─── Accents ────────────────────────────────────────────────────────────────

#[test]
fn accent_basic() {
    let s = AccentStrategy;
    assert_eq!(s.transform("Hello"), "Ĥéľľó");
}

#[test]
fn accent_empty() {
    let s = AccentStrategy;
    assert_eq!(s.transform(""), "");
}

#[test]
fn accent_unicode_passthrough() {
    let s = AccentStrategy;
    assert_eq!(s.transform("中文"), "中文");
}

#[test]
fn accent_numbers_unchanged() {
    let s = AccentStrategy;
    assert_eq!(s.transform("123"), "123");
}

#[test]
fn accent_whitespace_unchanged() {
    let s = AccentStrategy;
    assert_eq!(s.transform("  \t\n"), "  \t\n");
}

// ─── CJK ────────────────────────────────────────────────────────────────────

#[test]
fn cjk_basic() {
    let s = CjkStrategy;
    let result = s.transform("Hello World");
    assert!(result.len() > "Hello World".len());
}

#[test]
fn cjk_empty() {
    let s = CjkStrategy;
    assert_eq!(s.transform(""), "");
}

#[test]
fn cjk_already_unicode() {
    let s = CjkStrategy;
    let result = s.transform("日本語テスト");
    assert!(result.chars().count() > 6);
}

// ─── Special Chars ──────────────────────────────────────────────────────────

#[test]
fn special_basic() {
    let s = SpecialCharsStrategy;
    let result = s.transform("Hello World!");
    assert!(result.contains('§'));
}

#[test]
fn special_empty() {
    let s = SpecialCharsStrategy;
    assert_eq!(s.transform(""), "");
}

#[test]
fn special_already_unicode() {
    let s = SpecialCharsStrategy;
    let input = "日本語テストのテスト";
    let result = s.transform(input);
    assert!(result.chars().count() > input.chars().count());
}

// ─── Expansion ──────────────────────────────────────────────────────────────

#[test]
fn expansion_basic() {
    let s = ExpansionStrategy::new(1.5);
    let result = s.transform("Hello");
    // 5 * 1.5 = 7.5 -> 8 chars
    assert!(
        result.chars().count() >= 8,
        "Got: {} ({})",
        result,
        result.chars().count()
    );
}

#[test]
fn expansion_empty() {
    let s = ExpansionStrategy::new(2.0);
    assert_eq!(s.transform(""), "");
}

#[test]
fn expansion_ratio_one() {
    let s = ExpansionStrategy::new(1.0);
    assert_eq!(s.transform("Hello"), "Hello");
}

#[test]
fn expansion_already_unicode() {
    let s = ExpansionStrategy::new(1.5);
    let result = s.transform("ĥéľľó");
    assert!(result.chars().count() > 5);
}

// ─── Brackets ───────────────────────────────────────────────────────────────

#[test]
fn brackets_basic() {
    let s = BracketStrategy;
    assert_eq!(s.transform("Hello"), "[Hello]");
}

#[test]
fn brackets_empty() {
    let s = BracketStrategy;
    assert_eq!(s.transform(""), "");
}

#[test]
fn brackets_already_unicode() {
    let s = BracketStrategy;
    assert_eq!(s.transform("日本語"), "[日本語]");
}

// ─── RTL ────────────────────────────────────────────────────────────────────

#[test]
fn rtl_basic() {
    let s = RtlStrategy;
    let result = s.transform("Hello");
    assert!(result.starts_with('\u{202B}'));
    assert!(result.ends_with('\u{202C}'));
    assert!(result.contains("Hello"));
}

#[test]
fn rtl_empty() {
    let s = RtlStrategy;
    assert_eq!(s.transform(""), "");
}

#[test]
fn rtl_already_unicode() {
    let s = RtlStrategy;
    let result = s.transform("مرحبا");
    assert!(result.starts_with('\u{202B}'));
    assert!(result.ends_with('\u{202C}'));
}

// ─── Unicode Stress ─────────────────────────────────────────────────────────

#[test]
fn unicode_stress_basic() {
    let s = UnicodeStressStrategy;
    let result = s.transform("Hello World");
    assert!(result.len() > "Hello World".len());
}

#[test]
fn unicode_stress_empty() {
    let s = UnicodeStressStrategy;
    assert_eq!(s.transform(""), "");
}

#[test]
fn unicode_stress_already_unicode() {
    let s = UnicodeStressStrategy;
    let result = s.transform("日本語テスト");
    assert!(result.len() > "日本語テスト".len());
}

#[test]
fn unicode_stress_has_combining() {
    let s = UnicodeStressStrategy;
    let result = s.transform("abcdef");
    let has_combining = result
        .chars()
        .any(|c| ('\u{0300}'..='\u{036F}').contains(&c));
    assert!(
        has_combining,
        "Should contain combining diacritics: {result}"
    );
}
