use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct AnyPattern;

pub const ANY: AnyPattern = AnyPattern;

impl ReadPattern for AnyPattern {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        if let Some(ch) = text.chars().next() {
            Some(ch.len_utf8())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::pat;
    use super::*;
    use crate::ReadPattern;

    #[test]
    fn any_pattern() {
        assert!(ANY.test_pattern("q"));
        assert!(ANY.test_pattern("😀"));
        assert_eq!(ANY.read_pattern("😀"), Some('😀'.len_utf8()));
        assert!(ANY.test_pattern("Ф"));
        assert!(!ANY.test_pattern(""));

        let any_text = pat(ANY) * ..;
        assert!(any_text.test_pattern(""));
        assert!(any_text.test_pattern("🍏🍎🍐🍊🍋🍌"));
        assert!(any_text.test_pattern("Привет, мир!"));
    }
}
