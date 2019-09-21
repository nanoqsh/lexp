
use crate::read_pattern::ReadPattern;

#[derive(Copy, Clone, Debug)]
pub struct ManyPattern<T>(pub T, pub u32);

impl<T> ReadPattern for ManyPattern<T> where
    T: ReadPattern {
    fn read_pattern(&self, text: &str) -> Option<usize> {
        let mut len = 0;

        for _ in 0..self.1 {
            match self.0.read_pattern(&text[len..]) {
                Some(l) => len += l,
                None    => return None,
            }
        }

        Some(len)
    }
}
