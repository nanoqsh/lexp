
use crate::read_pattern::ReadPattern;
use crate::patterns::{OrPattern, AndPattern, RangePattern, ManyPattern, UntilPattern};
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

impl<T: ReadPattern> Pattern<T> {
    pub fn until<U: ReadPattern>(self, pattern: U) -> UntilPattern<T, U> {
        UntilPattern(self.0, pattern)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_str() {
        let pattern = Pattern("some text");
        assert_eq!(pattern.read_pattern("some text"), Some("some text".len()));
        assert_eq!(pattern.read_pattern("some text_"), Some("some text".len()));
        assert_eq!(pattern.read_pattern("text"), None);
        assert_eq!(pattern.read_pattern(""), None);
    }

    #[test]
    fn pattern_char() {
        let pattern = Pattern('a');
        assert_eq!(pattern.read_pattern("a"), Some(1));
        assert_eq!(pattern.read_pattern("ab"), Some(1));
        assert_eq!(pattern.read_pattern("b"), None);
    }

    #[test]
    fn combine_patterns() {
        let a = Pattern("a") | "b" | "c";
        let b = Pattern("0") & "" & "1";

        let ab = a & b;
        assert_eq!(ab.read_pattern("a01"), Some(3));
        assert_eq!(ab.read_pattern("b01"), Some(3));
        assert_eq!(ab.read_pattern("c01"), Some(3));
        assert_eq!(ab.read_pattern("c0"), None);

        let a_b = a | b;
        assert_eq!(a_b.read_pattern("a"), Some(1));
        assert_eq!(a_b.read_pattern("b"), Some(1));
        assert_eq!(a_b.read_pattern("c"), Some(1));
        assert_eq!(a_b.read_pattern("01"), Some(2));
        assert_eq!(a_b.read_pattern("0"), None);

        let aaa = a * 3;
        assert_eq!(aaa.read_pattern("aaa"), Some(3));
        assert_eq!(aaa.read_pattern("abc"), Some(3));
        assert_eq!(aaa.read_pattern("bac"), Some(3));

        let bb = b * 2;
        assert_eq!(bb.read_pattern("0101"), Some(4));
        assert_eq!(bb.read_pattern("01"), None);
        assert_eq!(bb.read_pattern("010101"), Some(4));

        let a_aaa = a * (1..=3);
        assert_eq!(a_aaa.read_pattern("b"), Some(1));
        assert_eq!(a_aaa.read_pattern("cb"), Some(2));
        assert_eq!(a_aaa.read_pattern("bca"), Some(3));
        assert_eq!(a_aaa.read_pattern("aabb"), Some(3));
        assert_eq!(a_aaa.read_pattern(""), None);

        let b_bb = b * (1..3);
        assert_eq!(b_bb.read_pattern("01"), Some(2));
        assert_eq!(b_bb.read_pattern("0101"), Some(4));
        assert_eq!(b_bb.read_pattern("010101"), Some(4));
        assert_eq!(b_bb.read_pattern(""), None);

        let not_a_3 = Pattern(|c: char| c != 'a') * 3;
        assert_eq!(not_a_3.read_pattern("zzz"), Some(3));
        assert_eq!(not_a_3.read_pattern("zz"), None);
        assert_eq!(not_a_3.read_pattern("zzzz"), Some(3));
        assert_eq!(not_a_3.read_pattern("aaa"), None);
        assert_eq!(not_a_3.read_pattern("zza"), None);

        let whitespace_or_alpha = Pattern(char::is_whitespace) | char::is_alphabetic;
        assert_eq!(whitespace_or_alpha.read_pattern("a"), Some(1));
        assert_eq!(whitespace_or_alpha.read_pattern(" "), Some(1));
        assert_eq!(whitespace_or_alpha.read_pattern("*"), None);

        let w_a_range = whitespace_or_alpha * (1..=2);
        assert_eq!(w_a_range.read_pattern("a"), Some(1));
        assert_eq!(w_a_range.read_pattern("a "), Some(2));
        assert_eq!(w_a_range.read_pattern(""), None);
        assert_eq!(w_a_range.read_pattern("z+"), Some(1));
        assert_eq!(w_a_range.read_pattern(" f "), Some(2));
    }
}
