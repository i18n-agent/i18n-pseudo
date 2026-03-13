use super::Strategy;

/// Insert CJK characters periodically (every ~5 chars).
pub struct CjkStrategy;

const CJK_CHARS: &[char] = &['中', '文', '韓', '国', '語', '日', '本'];

impl Strategy for CjkStrategy {
    fn transform(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(text.len() * 2);
        let mut char_count = 0usize;
        let mut cjk_idx = 0usize;

        for ch in text.chars() {
            result.push(ch);
            char_count += 1;

            if char_count % 5 == 0 {
                result.push(CJK_CHARS[cjk_idx % CJK_CHARS.len()]);
                cjk_idx += 1;
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
        let s = CjkStrategy;
        let result = s.transform("Hello World");
        assert!(result.contains('中'), "Should contain CJK char: {result}");
        assert!(result.len() > "Hello World".len());
    }

    #[test]
    fn empty_input() {
        let s = CjkStrategy;
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn short_input_no_cjk() {
        let s = CjkStrategy;
        // Less than 5 chars: no CJK inserted
        assert_eq!(s.transform("Hi"), "Hi");
    }

    #[test]
    fn exact_five_chars() {
        let s = CjkStrategy;
        let result = s.transform("abcde");
        assert_eq!(result, "abcde中");
    }

    #[test]
    fn already_unicode() {
        let s = CjkStrategy;
        let result = s.transform("日本語テスト");
        // Should still insert CJK chars periodically
        assert!(result.len() > "日本語テスト".len());
    }

    #[test]
    fn cycles_through_chars() {
        let s = CjkStrategy;
        // 15 chars => CJK at positions 5, 10, 15
        let result = s.transform("abcdeabcdeabcde");
        let cjk_count = result.chars().filter(|c| CJK_CHARS.contains(c)).count();
        assert_eq!(cjk_count, 3);
    }
}
