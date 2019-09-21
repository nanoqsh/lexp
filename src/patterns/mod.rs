
mod pattern;
mod or_pattern;
mod and_pattern;
mod range_pattern;
mod many_pattern;

pub use pattern::Pattern;
pub use pattern::pat;
pub use or_pattern::OrPattern;
pub use and_pattern::AndPattern;
pub use many_pattern::ManyPattern;
pub use range_pattern::RangePattern;
