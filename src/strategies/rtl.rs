use super::Strategy;

/// Wrap text with Unicode bidi markers for RTL testing.
/// U+202B (RIGHT-TO-LEFT EMBEDDING) before, U+202C (POP DIRECTIONAL FORMATTING) after.
pub struct RtlStrategy;

/// RIGHT-TO-LEFT EMBEDDING
const RLE: char = '\u{202B}';
/// POP DIRECTIONAL FORMATTING
const PDF: char = '\u{202C}';

impl Strategy for RtlStrategy {
    fn transform(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }
        format!("{RLE}{text}{PDF}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transform() {
        let s = RtlStrategy;
        let result = s.transform("Hello");
        assert!(result.starts_with('\u{202B}'));
        assert!(result.ends_with('\u{202C}'));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn empty_input() {
        let s = RtlStrategy;
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn already_unicode() {
        let s = RtlStrategy;
        let result = s.transform("مرحبا");
        assert!(result.starts_with('\u{202B}'));
        assert!(result.ends_with('\u{202C}'));
    }

    #[test]
    fn markers_present() {
        let s = RtlStrategy;
        let result = s.transform("Test");
        // Should be exactly RLE + "Test" + PDF
        assert_eq!(result.chars().count(), 4 + 2);
    }
}
