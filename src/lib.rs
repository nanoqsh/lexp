
#[cfg_attr(test, macro_use)]
extern crate read_token_derive;

pub use read_token_derive::ReadToken;

mod lexer;
mod parse;
mod read_token;
mod read_pattern;
pub mod patterns {
    mod pattern;
    mod or_pattern;
    mod and_pattern;
    mod range_pattern;
    mod many_pattern;

    pub use pattern::{Pattern, pat};
    pub use or_pattern::OrPattern;
    pub use and_pattern::AndPattern;
    pub use range_pattern::RangePattern;
    pub use many_pattern::ManyPattern;
}

pub use lexer::{lex, Lexer, Lexeme};
pub use parse::{ParseResult, Parse, ParseIterator};
pub use read_token::ReadToken;
pub use read_pattern::ReadPattern;
