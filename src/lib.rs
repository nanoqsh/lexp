#[cfg_attr(test, macro_use)]
extern crate read_token_derive;

pub use read_token_derive::ReadToken;

mod lexer;
mod parse;
mod read_pattern;
mod read_token;
pub mod patterns {
    mod and_pattern;
    mod any_pattern;
    mod many_pattern;
    mod or_pattern;
    mod pattern;
    mod range_pattern;
    mod until_pattern;

    pub use and_pattern::AndPattern;
    pub use any_pattern::{AnyPattern, ANY};
    pub use many_pattern::ManyPattern;
    pub use or_pattern::OrPattern;
    pub use pattern::{pat, Pattern};
    pub use range_pattern::RangePattern;
    pub use until_pattern::UntilPattern;
}

pub use lexer::{lex, Lexeme, Lexer};
pub use parse::{Parse, ParseIterator, ParseResult};
pub use read_pattern::ReadPattern;
pub use read_token::ReadToken;

mod tests;
