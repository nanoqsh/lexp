use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct AndPattern<L, R>(pub L, pub R);

impl<L, R> ReadPattern for AndPattern<L, R>
where
    L: ReadPattern,
    R: ReadPattern,
{
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let len_a = self.0.read_pattern(text)?;
        let len_b = self.1.read_pattern(&text[len_a..])?;
        Some(len_a + len_b)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;
    use crate::ReadPattern;

    #[test]
    fn and_pattern() {
        let pattern = Pattern("foo") & "bar";
        assert!(pattern.test_pattern("foobar"));
        assert!(!pattern.test_pattern("foo"));
        assert!(!pattern.test_pattern("bar"));
        assert!(!pattern.test_pattern(""));

        let a = Pattern("") & "a";
        assert!(a.test_pattern("a"));

        let b = Pattern("") & "b";
        assert!(b.test_pattern("b"));

        let empty_pattern = Pattern("") & "";
        assert!(empty_pattern.test_pattern(""));
        assert!(!empty_pattern.test_pattern("a"));
        assert!(!empty_pattern.test_pattern("b"));
    }
}
