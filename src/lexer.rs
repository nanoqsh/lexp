use super::parse::Parse;
use super::read_pattern::ReadPattern;
use super::read_token::ReadToken;
use std::ops::BitOr;

pub struct Lexeme<P, R> {
    read_pattern: P,
    read_token: R,
}

pub fn lex<'t, P, R>(read_pattern: P, read_token: R) -> Lexeme<P, R>
where
    P: ReadPattern,
    R: ReadToken<'t>,
{
    Lexeme {
        read_pattern,
        read_token,
    }
}

impl<'t, P, R> Parse<'t> for Lexeme<P, R>
where
    P: ReadPattern,
    R: ReadToken<'t>,
{
    type Token = R::Token;

    fn parse(&self, text: &'t str) -> Option<(Self::Token, usize)> {
        let len = self.read_pattern.read_pattern(text)?;
        let tok = self.read_token.read_token(&text[..len]);
        Some((tok, len))
    }
}

impl<'t, P, T, R> BitOr<R> for Lexeme<P, T>
where
    P: ReadPattern,
    T: ReadToken<'t>,
    R: Parse<'t>,
{
    type Output = Lexer<Lexeme<P, T>, R>;

    fn bitor(self, rhs: R) -> Self::Output {
        Lexer {
            left: self,
            right: rhs,
        }
    }
}

/// Lexer is a combination of lexemes
///
/// This struct allows to combine any lexemes together like:
/// `a | b | c`
///
pub struct Lexer<L, R> {
    left: L,
    right: R,
}

impl<'t, T, L, R> Parse<'t> for Lexer<L, R>
where
    L: Parse<'t, Token = T>,
    R: Parse<'t, Token = T>,
{
    type Token = T;

    fn parse(&self, text: &'t str) -> Option<(Self::Token, usize)> {
        self.left.parse(text).or_else(|| self.right.parse(text))
    }
}

impl<'t, L, R, P> BitOr<P> for Lexer<L, R>
where
    P: Parse<'t>,
{
    type Output = Lexer<Lexer<L, R>, P>;

    fn bitor(self, rhs: P) -> Self::Output {
        Lexer {
            left: self,
            right: rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Debug, ReadToken)]
    enum Token {
        PlusEq,
        Plus,
        Star,
        Name,
        Num,
        Eq,
        Semicolon,
    }

    #[test]
    fn lexeme() {
        let lx = lex("+=", Token::PlusEq);
        assert_eq!(lx.parse("+=").unwrap(), (Token::PlusEq, 2));
        assert!(lx.parse("*").is_none());
    }

    #[test]
    fn combine_lexemes() {
        let plus_or_star = lex("+", Token::Plus) | lex("*", Token::Star);
        assert_eq!(plus_or_star.parse("+").unwrap(), (Token::Plus, 1));
        assert_eq!(plus_or_star.parse("*").unwrap(), (Token::Star, 1));
        assert!(plus_or_star.parse(".").is_none());

        let l = lex("name", Token::Name)
            | lex("=", Token::Eq)
            | lex("1", Token::Num)
            | lex(";", Token::Semicolon);

        assert_eq!(l.parse("name").unwrap(), (Token::Name, 4));
        assert_eq!(l.parse("=").unwrap(), (Token::Eq, 1));
        assert_eq!(l.parse("1").unwrap(), (Token::Num, 1));
        assert_eq!(l.parse(";").unwrap(), (Token::Semicolon, 1));
        assert!(l.parse("!").is_none());
    }
}
