use super::Strategy;

/// Wrap text with square brackets: `[text]`.
pub struct BracketStrategy;

impl Strategy for BracketStrategy {
    fn transform(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }
        format!("[{text}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transform() {
        let s = BracketStrategy;
        assert_eq!(s.transform("Hello"), "[Hello]");
    }

    #[test]
    fn empty_input() {
        let s = BracketStrategy;
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn already_unicode() {
        let s = BracketStrategy;
        assert_eq!(s.transform("日本語"), "[日本語]");
    }

    #[test]
    fn nested_brackets() {
        let s = BracketStrategy;
        assert_eq!(s.transform("[inner]"), "[[inner]]");
    }
}
