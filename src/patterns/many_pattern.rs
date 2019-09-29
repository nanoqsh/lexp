use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct ManyPattern<T>(pub T, pub u32);

impl<T> ReadPattern for ManyPattern<T>
where
    T: ReadPattern,
{
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let mut len = 0;

        for _ in 0..self.1 {
            match self.0.read_pattern(&text[len..]) {
                Some(l) => len += l,
                None => return None,
            }
        }

        Some(len)
    }
}

#[cfg(test)]
mod tests {
    use super::super::pat;
    use crate::ReadPattern;

    #[test]
    fn many_pattern() {
        let pattern = pat("ab") * 3;
        assert!(pattern.test_pattern("ababab"));
        assert!(!pattern.test_pattern("aba"));
        assert!(!pattern.test_pattern("a"));
        assert!(!pattern.test_pattern("abab"));
        assert!(!pattern.test_pattern("abababab"));
        assert!(!pattern.test_pattern(""));

        let empty_pattern = pat("b") * 0;
        assert!(empty_pattern.test_pattern(""));

        let pattern = pat("z") * 4;
        assert!(!pattern.test_pattern(""));
        assert!(!pattern.test_pattern("z"));
        assert!(!pattern.test_pattern("zz"));
        assert!(!pattern.test_pattern("zzz"));
        assert!(pattern.test_pattern("zzzz"));
        assert!(!pattern.test_pattern("zzzzz"));
    }
}
