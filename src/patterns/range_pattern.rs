
use crate::read_pattern::ReadPattern;
use std::ops::{RangeBounds, Bound};

#[derive(Copy, Clone, Debug)]
pub struct RangePattern<T, R>(pub T, pub R);

impl<T, R> ReadPattern for RangePattern<T, R> where
    T: ReadPattern,
    R: RangeBounds<u32> {
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
                        Bound::Included(b) if *b == count     => return Some(len),
                        Bound::Excluded(b) if *b == count + 1 => return Some(len),
                        _ => {},
                    }
                },
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
        assert_eq!(full.read_pattern(""), Some(0));
        assert_eq!(full.read_pattern("a"), Some(1));
        assert_eq!(full.read_pattern("aa"), Some(2));
        assert_eq!(full.read_pattern("aaa"), Some(3));

        let from = pat("b") * (2..);
        assert_eq!(from.read_pattern(""), None);
        assert_eq!(from.read_pattern("b"), None);
        assert_eq!(from.read_pattern("bb"), Some(2));
        assert_eq!(from.read_pattern("bbb"), Some(3));

        let to = pat("c") * ..2;
        assert_eq!(to.read_pattern(""), Some(0));
        assert_eq!(to.read_pattern("c"), Some(1));
        assert_eq!(to.read_pattern("cc"), Some(1));

        let to_inclusive = pat("d") * ..=2;
        assert_eq!(to_inclusive.read_pattern(""), Some(0));
        assert_eq!(to_inclusive.read_pattern("d"), Some(1));
        assert_eq!(to_inclusive.read_pattern("dd"), Some(2));
        assert_eq!(to_inclusive.read_pattern("ddd"), Some(2));

        let range = pat("e") * (1..3);
        assert_eq!(range.read_pattern(""), None);
        assert_eq!(range.read_pattern("e"), Some(1));
        assert_eq!(range.read_pattern("ee"), Some(2));
        assert_eq!(range.read_pattern("eee"), Some(2));

        let range_inclusive = pat("f") * (1..=2);
        assert_eq!(range_inclusive.read_pattern(""), None);
        assert_eq!(range_inclusive.read_pattern("f"), Some(1));
        assert_eq!(range_inclusive.read_pattern("ff"), Some(2));
        assert_eq!(range_inclusive.read_pattern("fff"), Some(2));
    }
}
