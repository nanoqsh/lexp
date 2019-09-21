
use std::ops::{Range, RangeInclusive};

pub trait ReadPattern {
    fn read_pattern(&self, text: &str) -> Option<usize>;
}

impl ReadPattern for &str {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        if text.starts_with(self) {
            Some(self.len())
        } else {
            None
        }
    }
}

impl ReadPattern for String {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        self.as_str().read_pattern(text)
    }
}

impl ReadPattern for char {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        if text.starts_with(*self) {
            Some(self.len_utf8())
        }
        else {
            None
        }
    }
}

impl<F: Fn(char) -> bool> ReadPattern for F {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        match text.chars().next() {
            Some(ch) if self(ch) => Some(ch.len_utf8()),
            _ => None,
        }
    }
}

impl ReadPattern for Range<char> {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let ch = text.chars().next()?;
        if self.contains(&ch) {
            Some(ch.len_utf8())
        }
        else {
            None
        }
    }
}

impl ReadPattern for RangeInclusive<char> {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let ch = text.chars().next()?;
        if self.contains(&ch) {
            Some(ch.len_utf8())
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_pattern_str() {
        let pattern = "some text";
        let same = "some text";
        assert_eq!(pattern.read_pattern(same), Some(same.len()));

        let part = "some text more";
        assert_eq!(pattern.read_pattern(part), Some(pattern.len()));

        let another = "text";
        assert_eq!(pattern.read_pattern(another), None);
    }

    #[test]
    fn read_pattern_str_utf8() {
        let pattern = "привет";
        assert_eq!(pattern.read_pattern("привет"), Some(pattern.len()));
        assert_eq!(pattern.read_pattern("привет, мир!"), Some(pattern.len()));
        assert_eq!(pattern.read_pattern("прив"), None);
    }

    #[test]
    fn read_pattern_empty_str() {
        let empty_pattern = "";
        let text = "text";
        assert_eq!(empty_pattern.read_pattern(text), Some(0));
        assert_eq!(empty_pattern.read_pattern(""), Some(0));

        let pattern = "baz";
        assert_eq!(pattern.read_pattern(""), None);
    }

    #[test]
    fn read_pattern_empty_string() {
        let pattern = String::from("some text");
        assert_eq!(pattern.read_pattern("some text"), Some(pattern.len()));
        assert_eq!(pattern.read_pattern("some text more"), Some(pattern.len()));
        assert_eq!(pattern.read_pattern("text"), None);
    }

    #[test]
    fn read_pattern_char() {
        let pattern = 'a';
        assert_eq!(pattern.read_pattern("a"), Some(1));
        assert_eq!(pattern.read_pattern("ab"), Some(1));
        assert_eq!(pattern.read_pattern("b"), None);
        assert_eq!(pattern.read_pattern(""), None);
    }

    #[test]
    fn read_pattern_char_utf8() {
        let pattern = 'ф';
        assert_eq!(pattern.read_pattern("ф"), Some(pattern.len_utf8()));
        assert_eq!(pattern.read_pattern("фы"), Some(pattern.len_utf8()));
        assert_eq!(pattern.read_pattern("ы"), None);
    }

    #[test]
    fn read_pattern_fn() {
        let whitespace = char::is_whitespace;
        assert_eq!(whitespace.read_pattern(" "), Some(1));
        assert_eq!(whitespace.read_pattern(""), None);
        assert_eq!(whitespace.read_pattern("."), None);

        let alpha = char::is_alphabetic;
        assert_eq!(alpha.read_pattern("a"), Some(1));
        assert_eq!(alpha.read_pattern("."), None);

        let a_or_b = |c: char| c == 'a' || c == 'b';
        assert_eq!(a_or_b.read_pattern("a"), Some(1));
        assert_eq!(a_or_b.read_pattern("b"), Some(1));
        assert_eq!(a_or_b.read_pattern("c"), None);
    }

    #[test]
    fn read_pattern_range() {
        let alpha = 'a'..='z';
        assert_eq!(alpha.read_pattern("a"), Some(1));
        assert_eq!(alpha.read_pattern("$"), None);
        assert_eq!(alpha.read_pattern("1"), None);

        let binary = '0'..'2';
        assert_eq!(binary.read_pattern("0"), Some(1));
        assert_eq!(binary.read_pattern("1"), Some(1));
        assert_eq!(binary.read_pattern("2"), None);
    }
}
