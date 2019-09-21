
use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct OrPattern<L, R>(pub L, pub R);

impl<L, R> ReadPattern for OrPattern<L, R> where
    L: ReadPattern,
    R: ReadPattern {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        match self.0.read_pattern(text) {
            None => self.1.read_pattern(text),
            s    => s,
        }
    }
}
