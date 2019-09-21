
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
