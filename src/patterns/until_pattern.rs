
use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct UntilPattern<P, U>(pub P, pub U);

impl<P, U> ReadPattern for UntilPattern<P, U> where
    P: ReadPattern,
    U: ReadPattern {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let mut len = 0;

        loop {
            let rest = &text[len..];
            match self.1.read_pattern(rest) {
                None          => len += self.0.read_pattern(rest)?,
                Some(end_len) => break Some(len + end_len),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::pat;
    use crate::ReadPattern;
    use crate::patterns::ANY;

    #[test]
    fn until_use() {
        // What if we need parse something like: / any text /
        // Just any text in slashes
        // Let's create pattern like:
        let slashed_text = pat('/') & (pat(ANY) * ..) & '/';
        assert!(!slashed_text.test_pattern("/test/")); // It fails this test

        // (pat(ANY) * ..) parses all text to the end
        // and therefore there is no place for parsing the last "/".
        // To fix this there is UntilPattern:
        let slashed_text = pat('/') & pat(ANY).until('/');
        // Now we don't need to specify the quantity
        assert!(slashed_text.test_pattern("/test/")); // Test passes

        // It also passes:
        assert!(slashed_text.test_pattern("//"));
        // What if we don't want this?
        // We want to have at least one entry in the middle.
        // Then the pattern could be like that:
        let slashed_text = pat('/') & pat(ANY) & pat(ANY).until('/');
        assert!(!slashed_text.test_pattern("//")); // It fails
        assert!(slashed_text.test_pattern("/a/")); // And it passes
    }

    #[test]
    fn until_pattern() {
        let p = pat("a").until(".");
        assert!(p.test_pattern("aaa."));
        assert!(p.test_pattern("."));
        assert!(!p.test_pattern("aaa"));
        assert!(!p.test_pattern("b."));

        let bin = pat('1') | '0';
        let parentheses_bin = pat('(') & bin.until(')');
        assert!(parentheses_bin.test_pattern("()"));
        assert!(parentheses_bin.test_pattern("(1)"));
        assert!(parentheses_bin.test_pattern("(0)"));
        assert!(parentheses_bin.test_pattern("(10)"));
        assert!(!parentheses_bin.test_pattern("(0"));
        assert!(!parentheses_bin.test_pattern("0)"));
        assert!(!parentheses_bin.test_pattern("("));
        assert!(!parentheses_bin.test_pattern(")"));
    }
}
