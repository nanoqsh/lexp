# lexp

## Example
```rust
let alpha = pat('a'..='z') | ('A'..='Z');
let name = alpha * (1..);
let number = pat('0'..='9') * (1..);
let comment = pat("/*") & pat(ANY).until("*/");

let lx =
      lex(' ', Token::Empty)
    | lex('\n', Token::NewLine)
    | lex('+', Token::Plus)
    | lex('*', Token::Star)
    | lex('=', Token::Eq)
    | lex('(', Token::LeftBracket)
    | lex(')', Token::RightBracket)
    | lex(';', Token::Semicolon)
    | lex(number, |n, _| Token::Number(u32::from_str(n).unwrap()))
    | lex(name, |n, _| match n {
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        n => Token::Name(n),
    })
    | lex(comment, |n, _| Token::Comment(n));

let code = String::from(
    "let x = 10;
    /* 🦄 */
    if (x = 12) x * 4;
    else x + 1;",
);

let tokens: Vec<Token> = lx
    .tokenize(code.as_str())
    .map(|r| match r {
        ParseResult::Ok(tok, _) => tok,
        ParseResult::UnexpectedAt(_) => unreachable!(),
    })
    .filter(|t| *t != Token::Empty)
    .collect();

assert_eq!(
    tokens,
    [
        Token::Let,                 // let
        Token::Name("x"),           // x
        Token::Eq,                  // =
        Token::Number(10),          // 10
        Token::Semicolon,           // ;
        Token::NewLine,
        Token::Comment("/* 🦄 */"), // /* 🦄 */
        Token::NewLine,
        Token::If,                  // if
        Token::LeftBracket,         // (
        Token::Name("x"),           // x
        Token::Eq,                  // =
        Token::Number(12),          // 12
        Token::RightBracket,        // )
        Token::Name("x"),           // x
        Token::Star,                // *
        Token::Number(4),           // 4
        Token::Semicolon,           // ;
        Token::NewLine,
        Token::Else,                // else
        Token::Name("x"),           // x
        Token::Plus,                // +
        Token::Number(1),           // 1
        Token::Semicolon,           // ;
    ]
);
```
