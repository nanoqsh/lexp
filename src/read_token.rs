pub trait ReadToken<'t> {
    type Token;
    fn read_token(&self, text: &'t str) -> Self::Token;
}

impl<'t, T, F> ReadToken<'t> for F
where
    T: ReadToken<'t>,
    F: Fn(&'t str) -> T,
{
    type Token = T;

    fn read_token(&self, text: &'t str) -> Self::Token {
        self(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Debug, ReadToken)]
    enum Token {
        A,
        B,
        Undefined,
    }

    #[test]
    fn read_token_derive() {
        assert_eq!(Token::A.read_token(""), Token::A);
        assert_eq!(Token::B.read_token(""), Token::B);
    }

    #[test]
    fn read_token_fn() {
        let rt = |text: &str| match text {
            "A" => Token::A,
            "B" => Token::B,
            _ => Token::Undefined,
        };

        assert_eq!(rt.read_token("A"), Token::A);
        assert_eq!(rt.read_token("B"), Token::B);
        assert_eq!(rt.read_token("C"), Token::Undefined);
    }

    #[derive(Copy, Clone, PartialEq, Debug, ReadToken)]
    enum TokenLT<'t> {
        Text(&'t str),
    }

    fn rt(text: &str) -> TokenLT {
        match text {
            t => TokenLT::Text(t),
        }
    }

    #[test]
    fn read_token_fn_lt() {
        assert_eq!(rt.read_token("text"), TokenLT::Text("text"));
    }
}
