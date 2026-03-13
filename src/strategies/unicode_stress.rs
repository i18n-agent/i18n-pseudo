use super::Strategy;

/// Add combining diacritics, zero-width joiners, and emoji sequences periodically
/// to stress-test Unicode handling.
pub struct UnicodeStressStrategy;

/// Combining diacritics (U+0300-U+036F range, selected subset)
const COMBINING_DIACRITICS: &[char] = &[
    '\u{0300}', // combining grave accent
    '\u{0301}', // combining acute accent
    '\u{0302}', // combining circumflex
    '\u{0303}', // combining tilde
    '\u{0308}', // combining diaeresis
    '\u{030A}', // combining ring above
    '\u{030B}', // combining double acute
    '\u{030C}', // combining caron
    '\u{0327}', // combining cedilla
    '\u{0328}', // combining ogonek
];

/// Zero-width joiner
const ZWJ: char = '\u{200D}';

/// Emoji sequences to insert periodically
const EMOJI_SEQUENCES: &[&str] = &[
    "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}", // family
    "\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}",          // rainbow flag
    "\u{1F469}\u{200D}\u{1F4BB}",                  // woman technologist
];

impl Strategy for UnicodeStressStrategy {
    fn transform(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(text.len() * 3);
        let mut char_count = 0usize;
        let mut diacritic_idx = 0usize;
        let mut emoji_idx = 0usize;

        for ch in text.chars() {
            result.push(ch);
            char_count += 1;

            // Add combining diacritic every 3 chars
            if char_count % 3 == 0 && ch.is_alphanumeric() {
                result.push(COMBINING_DIACRITICS[diacritic_idx % COMBINING_DIACRITICS.len()]);
                diacritic_idx += 1;
            }

            // Add ZWJ every 8 chars
            if char_count % 8 == 0 {
                result.push(ZWJ);
            }

            // Add emoji sequence every 12 chars
            if char_count % 12 == 0 {
                result.push_str(EMOJI_SEQUENCES[emoji_idx % EMOJI_SEQUENCES.len()]);
                emoji_idx += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transform() {
        let s = UnicodeStressStrategy;
        let result = s.transform("Hello World");
        assert!(result.len() > "Hello World".len());
    }

    #[test]
    fn empty_input() {
        let s = UnicodeStressStrategy;
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn contains_combining_diacritics() {
        let s = UnicodeStressStrategy;
        let result = s.transform("abcdef");
        // After char 3 and 6, we should see combining diacritics
        let has_combining = result
            .chars()
            .any(|c| ('\u{0300}'..='\u{036F}').contains(&c));
        assert!(
            has_combining,
            "Should contain combining diacritics: {result}"
        );
    }

    #[test]
    fn contains_zwj() {
        let s = UnicodeStressStrategy;
        let result = s.transform("abcdefghijklmnop"); // 16 chars
        let has_zwj = result.chars().any(|c| c == '\u{200D}');
        assert!(has_zwj, "Should contain ZWJ: {result}");
    }

    #[test]
    fn already_unicode_input() {
        let s = UnicodeStressStrategy;
        let result = s.transform("日本語テスト");
        assert!(result.len() > "日本語テスト".len());
    }
}
