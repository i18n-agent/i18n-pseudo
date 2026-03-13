use super::Strategy;

/// Replace ASCII Latin characters with accented Unicode equivalents.
pub struct AccentStrategy;

impl Strategy for AccentStrategy {
    fn transform(&self, text: &str) -> String {
        text.chars().map(accent_char).collect()
    }
}

fn accent_char(c: char) -> char {
    match c {
        'a' => '\u{00E1}', // á
        'b' => '\u{0180}', // ƀ
        'c' => '\u{00E7}', // ç
        'd' => '\u{00F0}', // ð
        'e' => '\u{00E9}', // é
        'f' => '\u{0192}', // ƒ
        'g' => '\u{01F5}', // ǵ
        'h' => '\u{0125}', // ĥ
        'i' => '\u{00ED}', // í
        'j' => '\u{0135}', // ĵ
        'k' => '\u{0199}', // ƙ
        'l' => '\u{013E}', // ľ
        'm' => '\u{0271}', // ɱ
        'n' => '\u{00F1}', // ñ
        'o' => '\u{00F3}', // ó
        'p' => '\u{00FE}', // þ
        'q' => '\u{A757}', // ꝗ
        'r' => '\u{0155}', // ŕ
        's' => '\u{0161}', // š
        't' => '\u{0165}', // ť
        'u' => '\u{00FA}', // ú
        'v' => '\u{1E7D}', // ṽ
        'w' => '\u{1E83}', // ẃ
        'x' => '\u{1E8B}', // ẋ
        'y' => '\u{00FD}', // ý
        'z' => '\u{017E}', // ž
        'A' => '\u{00C1}', // Á
        'B' => '\u{0181}', // Ɓ
        'C' => '\u{00C7}', // Ç
        'D' => '\u{00D0}', // Ð
        'E' => '\u{00C9}', // É
        'F' => '\u{0191}', // Ƒ
        'G' => '\u{01F4}', // Ǵ
        'H' => '\u{0124}', // Ĥ
        'I' => '\u{00CD}', // Í
        'J' => '\u{0134}', // Ĵ
        'K' => '\u{0198}', // Ƙ
        'L' => '\u{013D}', // Ľ
        'M' => '\u{1E40}', // Ṁ (closest uppercase accented M)
        'N' => '\u{00D1}', // Ñ
        'O' => '\u{00D3}', // Ó
        'P' => '\u{00DE}', // Þ
        'Q' => '\u{A756}', // Ꝗ
        'R' => '\u{0154}', // Ŕ
        'S' => '\u{0160}', // Š
        'T' => '\u{0164}', // Ť
        'U' => '\u{00DA}', // Ú
        'V' => '\u{1E7C}', // Ṽ
        'W' => '\u{1E82}', // Ẃ
        'X' => '\u{1E8A}', // Ẋ
        'Y' => '\u{00DD}', // Ý
        'Z' => '\u{017D}', // Ž
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transform() {
        let s = AccentStrategy;
        assert_eq!(s.transform("Hello"), "Ĥéľľó");
    }

    #[test]
    fn empty_input() {
        let s = AccentStrategy;
        assert_eq!(s.transform(""), "");
    }

    #[test]
    fn non_ascii_passthrough() {
        let s = AccentStrategy;
        // Already-unicode characters should pass through unchanged
        assert_eq!(s.transform("中文"), "中文");
    }

    #[test]
    fn mixed_case() {
        let s = AccentStrategy;
        let result = s.transform("AbCd");
        assert_eq!(result, "ÁƀÇð");
    }

    #[test]
    fn numbers_and_punctuation_unchanged() {
        let s = AccentStrategy;
        assert_eq!(s.transform("123!@#"), "123!@#");
    }

    #[test]
    fn full_alphabet() {
        let s = AccentStrategy;
        let lower = s.transform("abcdefghijklmnopqrstuvwxyz");
        // Every char should be different from the original
        for (orig, transformed) in "abcdefghijklmnopqrstuvwxyz".chars().zip(lower.chars()) {
            assert_ne!(orig, transformed, "char '{orig}' was not transformed");
        }
    }
}
