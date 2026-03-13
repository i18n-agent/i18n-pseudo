use super::Strategy;

/// Expand text length by a configurable ratio (1.0-3.0).
/// Pads by repeating vowels in the text and appending padding characters.
pub struct ExpansionStrategy {
    ratio: f64,
}

impl ExpansionStrategy {
    pub fn new(ratio: f64) -> Self {
        Self {
            ratio: ratio.clamp(1.0, 3.0),
        }
    }
}

const PADDING_CHARS: &[char] = &['~', '-', '.', ' '];

impl Strategy for ExpansionStrategy {
    fn transform(&self, text: &str) -> String {
        if text.is_empty() || self.ratio <= 1.0 {
            return text.to_string();
        }

        let original_len = text.chars().count();
        let target_len = (original_len as f64 * self.ratio).ceil() as usize;
        let extra_needed = target_len.saturating_sub(original_len);

        if extra_needed == 0 {
            return text.to_string();
        }

        // First pass: expand by doubling vowels in the text
        let mut result = String::with_capacity(target_len * 4); // generous for UTF-8
        let mut added = 0usize;

        for ch in text.chars() {
            result.push(ch);
            if added < extra_needed && is_vowel(ch) {
                result.push(ch);
                added += 1;
            }
        }

        // Second pass: if still short, append padding characters
        let mut pad_idx = 0usize;
        while added < extra_needed {
            result.push(PADDING_CHARS[pad_idx % PADDING_CHARS.len()]);
            pad_idx += 1;
            added += 1;
        }

        result
    }
}

fn is_vowel(c: char) -> bool {
    matches!(
        c.to_ascii_lowercase(),
        'a' | 'e' | 'i' | 'o' | 'u'
            // Also handle common accented vowels
            | '\u{00E1}' | '\u{00E9}' | '\u{00ED}' | '\u{00F3}' | '\u{00FA}'
            | '\u{00C1}' | '\u{00C9}' | '\u{00CD}' | '\u{00D3}' | '\u{00DA}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_expansion() {
        let s = ExpansionStrategy::new(1.3);
        let result = s.transform("Hello");
        // 5 chars * 1.3 = 6.5, ceil = 7 chars needed
        assert!(
            result.chars().count() >= 7,
            "Expected at least 7 chars, got {} ('{}')",
            result.chars().count(),
            result
        );
    }

    #[test]
    fn empty_input() {
        let s = ExpansionStrategy::new(1.5);
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn ratio_one_no_change() {
        let s = ExpansionStrategy::new(1.0);
        assert_eq!(s.transform("Hello"), "Hello");
    }

    #[test]
    fn high_ratio() {
        let s = ExpansionStrategy::new(3.0);
        let result = s.transform("Hi");
        // 2 * 3.0 = 6 chars
        assert!(
            result.chars().count() >= 6,
            "Expected at least 6 chars, got {} ('{}')",
            result.chars().count(),
            result
        );
    }

    #[test]
    fn no_vowels_uses_padding() {
        let s = ExpansionStrategy::new(2.0);
        let result = s.transform("xyz");
        // 3 * 2.0 = 6 chars, no vowels to expand, so padding appended
        assert!(result.chars().count() >= 6);
        assert!(result.starts_with("xyz"));
    }

    #[test]
    fn clamped_to_max() {
        let s = ExpansionStrategy::new(5.0);
        // Should clamp to 3.0
        let result = s.transform("ab");
        // 2 * 3.0 = 6
        assert!(result.chars().count() >= 6);
    }

    #[test]
    fn already_unicode_input() {
        let s = ExpansionStrategy::new(1.5);
        let result = s.transform("ĥéľľó");
        assert!(result.chars().count() > 5);
    }
}
