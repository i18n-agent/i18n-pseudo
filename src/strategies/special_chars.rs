use super::Strategy;

/// Insert special characters periodically (every ~7 chars).
pub struct SpecialCharsStrategy;

const SPECIAL_CHARS: &[char] = &[
    '§', '¶', '†', '‡', '©', '®', '™', '€', '£', '¥', '←', '→', '↑', '↓', '✓', '✗', '★', '☆', '♠',
    '♣', '♥', '♦',
];

impl Strategy for SpecialCharsStrategy {
    fn transform(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(text.len() * 2);
        let mut char_count = 0usize;
        let mut special_idx = 0usize;

        for ch in text.chars() {
            result.push(ch);
            char_count += 1;

            if char_count.is_multiple_of(7) {
                result.push(SPECIAL_CHARS[special_idx % SPECIAL_CHARS.len()]);
                special_idx += 1;
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
        let s = SpecialCharsStrategy;
        let result = s.transform("Hello World!");
        assert!(
            result.contains('§'),
            "Should contain special char: {result}"
        );
    }

    #[test]
    fn empty_input() {
        let s = SpecialCharsStrategy;
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn short_input_no_special() {
        let s = SpecialCharsStrategy;
        assert_eq!(s.transform("Hi"), "Hi");
    }

    #[test]
    fn exact_seven_chars() {
        let s = SpecialCharsStrategy;
        let result = s.transform("abcdefg");
        assert_eq!(result, "abcdefg§");
    }

    #[test]
    fn already_unicode() {
        let s = SpecialCharsStrategy;
        let result = s.transform("日本語テストのテスト");
        // 10 chars, should insert at position 7
        assert!(result.chars().any(|c| SPECIAL_CHARS.contains(&c)));
    }
}
