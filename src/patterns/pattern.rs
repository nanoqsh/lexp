
use crate::read_pattern::ReadPattern;
use crate::patterns::{OrPattern, AndPattern, RangePattern, ManyPattern};
use std::ops::{
    BitOr,
    BitAnd,
    Mul,
    Range,
    RangeInclusive,
    RangeToInclusive,
    RangeFrom,
    RangeTo,
    RangeFull
};

#[derive(Copy, Clone, Debug)]
pub struct Pattern<T>(pub T);

pub fn pat<T: ReadPattern>(pattern: T) -> Pattern<T> {
    Pattern(pattern)
}

impl<T> ReadPattern for Pattern<T> where
    T: ReadPattern {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        self.0.read_pattern(text)
    }
}

impl<L, R> BitOr<R> for Pattern<L> where
    L: ReadPattern,
    R: ReadPattern {
    type Output = Pattern<OrPattern<L, R>>;

    fn bitor(self, rhs: R) -> Self::Output {
        Pattern(OrPattern(self.0, rhs))
    }
}

impl<L, R> BitAnd<R> for Pattern<L> where
    L: ReadPattern,
    R: ReadPattern {
    type Output = Pattern<AndPattern<L, R>>;

    fn bitand(self, rhs: R) -> Self::Output {
        Pattern(AndPattern(self.0, rhs))
    }
}

impl<T> Mul<u32> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<ManyPattern<T>>;

    fn mul(self, rhs: u32) -> Self::Output {
        Pattern(ManyPattern(self.0, rhs))
    }
}

impl<T> Mul<RangeFull> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<RangePattern<T, RangeFull>>;

    fn mul(self, rhs: RangeFull) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

impl<T> Mul<RangeFrom<u32>> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<RangePattern<T, RangeFrom<u32>>>;

    fn mul(self, rhs: RangeFrom<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

impl<T> Mul<RangeTo<u32>> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<RangePattern<T, RangeTo<u32>>>;

    fn mul(self, rhs: RangeTo<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

impl<T> Mul<RangeToInclusive<u32>> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<RangePattern<T, RangeToInclusive<u32>>>;

    fn mul(self, rhs: RangeToInclusive<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

impl<T> Mul<Range<u32>> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<RangePattern<T, Range<u32>>>;

    fn mul(self, rhs: Range<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

impl<T> Mul<RangeInclusive<u32>> for Pattern<T> where
    T: ReadPattern {
    type Output = Pattern<RangePattern<T, RangeInclusive<u32>>>;

    fn mul(self, rhs: RangeInclusive<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}
