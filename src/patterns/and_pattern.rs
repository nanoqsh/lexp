
use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct AndPattern<L, R>(pub L, pub R);

impl<L, R> ReadPattern for AndPattern<L, R> where
    L: ReadPattern,
    R: ReadPattern {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let len_a = self.0.read_pattern(text)?;
        let len_b = self.1.read_pattern(&text[len_a..])?;
        Some(len_a + len_b)
    }
}
