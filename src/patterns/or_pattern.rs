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
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;
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
}
