use crate::read_pattern::ReadPattern;
use std::ops::{Bound, RangeBounds};

#[derive(Copy, Clone, Debug)]
pub struct RangePattern<T, R>(pub T, pub R);

impl<T, R> ReadPattern for RangePattern<T, R>
where
    T: ReadPattern,
    R: RangeBounds<u32>,
{
    fn read_pattern(&self, text: &str) -> Option<usize> {
        if self.0.read_pattern("").is_some() && self.1.end_bound() == Bound::Unbounded {
            panic!("Infinity loop")
        }

        let mut len = 0;
        let mut count = 0;

        loop {
            match self.0.read_pattern(&text[len..]) {
                Some(l) => {
                    len += l;
                    count += 1;

                    match self.1.end_bound() {
                        Bound::Included(b) if *b == count => return Some(len),
                        Bound::Excluded(b) if *b == count + 1 => return Some(len),
                        _ => {}
                    }
                }
                None if self.1.contains(&count) => return Some(len),
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::pat;
    use crate::ReadPattern;

    #[test]
    fn range_pattern() {
        let full = pat("a") * ..;
        assert!(full.test_pattern(""));
        assert!(full.test_pattern("a"));
        assert!(full.test_pattern("aa"));
        assert!(full.test_pattern("aaa"));

        let from = pat("b") * (2..);
        assert!(!from.test_pattern(""));
        assert!(!from.test_pattern("b"));
        assert!(from.test_pattern("bb"));
        assert!(from.test_pattern("bbb"));

        let to = pat("c") * ..2;
        assert!(to.test_pattern(""));
        assert!(to.test_pattern("c"));
        assert!(!to.test_pattern("cc"));

        let to_inclusive = pat("d") * ..=2;
        assert!(to_inclusive.test_pattern(""));
        assert!(to_inclusive.test_pattern("d"));
        assert!(to_inclusive.test_pattern("dd"));
        assert!(!to_inclusive.test_pattern("ddd"));

        let range = pat("e") * (1..3);
        assert!(!range.test_pattern(""));
        assert!(range.test_pattern("e"));
        assert!(range.test_pattern("ee"));
        assert!(!range.test_pattern("eee"));

        let range_inclusive = pat("f") * (1..=2);
        assert!(!range_inclusive.test_pattern(""));
        assert!(range_inclusive.test_pattern("f"));
        assert!(range_inclusive.test_pattern("ff"));
        assert!(!range_inclusive.test_pattern("fff"));
    }
}
