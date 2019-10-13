use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct OrPattern<L, R>(pub L, pub R);

impl<L, R> ReadPattern for OrPattern<L, R>
where
    L: ReadPattern,
    R: ReadPattern,
{
    fn read_pattern(&self, text: &str) -> Option<usize> {
        match self.0.read_pattern(text) {
            None => self.1.read_pattern(text),
            s => s,
        }
    }

    fn read_pattern_caps<'t>(&self, text: &'t str, buf: &mut Vec<&'t str>) -> Option<usize> {
        let mut add = Vec::new();
        let result = match self.0.read_pattern_caps(text, &mut add) {
            None => self.1.read_pattern_caps(text, &mut add),
            s => s,
        };

        buf.append(&mut add);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;
    use crate::patterns::{cap, pat};
    use crate::ReadPattern;

    #[test]
    fn or_pattern() {
        let pattern = Pattern("foo") | "bar";
        assert!(pattern.test_pattern("foo"));
        assert!(pattern.test_pattern("bar"));
        assert!(!pattern.test_pattern("baz"));
        assert!(!pattern.test_pattern(""));

        let a = Pattern("a") | "";
        assert!(a.test_pattern(""));
        assert!(a.test_pattern("a"));
        assert!(!a.test_pattern("b"));

        let b = Pattern("") | "b";
        assert!(b.test_pattern(""));

        // This pattern will match "" first
        // thus "b" will never match
        assert!(!b.test_pattern("b"));
        assert!(!b.test_pattern("a"));

        let empty_pattern = Pattern("") | "";
        assert!(empty_pattern.test_pattern(""));
        assert!(!empty_pattern.test_pattern("x"));
    }

    #[test]
    fn or_pattern_caps() {
        let pattern = pat(cap(Pattern("foo"))) | cap("bar");
        let mut caps = Vec::new();

        assert_eq!(pattern.read_pattern_caps("foo", &mut caps), Some(3));
        assert_eq!(pattern.read_pattern_caps("bar", &mut caps), Some(3));
        assert_eq!(caps, ["foo", "bar"]);
    }
}
