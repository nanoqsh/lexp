use crate::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct Capture<T>(pub T);

impl<T> ReadPattern for Capture<T>
where
    T: ReadPattern,
{
    fn read_pattern(&self, text: &str) -> Option<usize> {
        self.0.read_pattern(text)
    }

    fn read_captures<'t>(&self, text: &'t str, buf: &mut Vec<&'t str>) -> Option<usize> {
        let len = self.0.read_pattern(text)?;
        buf.push(&text[..len]);
        Some(len)
    }
}

pub fn cap<T: ReadPattern>(pattern: T) -> Capture<T> {
    Capture(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::pat;

    #[test]
    fn capture_str() {
        let pattern = Capture("text");
        let mut caps = Vec::new();

        assert_eq!(pattern.read_pattern("text"), Some("text".len()));
        assert_eq!(pattern.read_captures("text", &mut caps), Some("text".len()));
        assert_eq!(caps, ["text"]);
    }

    #[test]
    fn capture_char() {
        let pattern = pat(cap('a'));
        let mut caps = Vec::new();

        assert_eq!(pattern.read_pattern("a"), Some(1));
        assert_eq!(pattern.read_captures("a", &mut caps), Some(1));
        assert_eq!(caps, ["a"]);
    }
}
