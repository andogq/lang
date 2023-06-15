use crate::{
    lexer::cursor::Cursor,
    token::{Keyword, LiteralKind, Token, TokenKind},
};

pub mod cursor;

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(source);

    std::iter::from_fn(move || {
        cursor.next().map(|(c, position)| {
            Token::new(
                match c {
                    '=' => TokenKind::Equals,
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Asterix,
                    '^' => TokenKind::Hat,
                    ';' => TokenKind::Semi,
                    '/' if cursor.peek_next().map(|c| c == '/').unwrap_or_default() => {
                        // Skip next `/`
                        cursor.next();

                        TokenKind::Comment(String::from_iter(cursor.take_while(|c| c != '\n')))
                    }
                    '/' => TokenKind::Slash,
                    '(' => TokenKind::LSmooth,
                    ')' => TokenKind::RSmooth,
                    c if c.is_ascii_whitespace() => {
                        // Consume through to the end of whitespace
                        cursor.skip_while(|c| c.is_ascii_whitespace());

                        TokenKind::Whitespace
                    }
                    c if c.is_ascii_digit() => TokenKind::Literal {
                        kind: LiteralKind::Integer,
                        chars: cursor.retake_while(|c| c.is_ascii_digit()),
                    },
                    c if is_ident_char(c) => {
                        let ident_str = String::from_iter(cursor.retake_while(is_ident_char));
                        match ident_str.as_str() {
                            // Match for keywords
                            "let" => TokenKind::Keyword(Keyword::Let),
                            _ => TokenKind::Identifier(ident_str),
                        }
                    }
                    '"' => {
                        let chars = cursor.take_while_config(false, false, |c, escaped| {
                            if c == '\\' {
                                // Backslash can escape itself
                                (true, !escaped)
                            } else if c == '"' && !escaped {
                                // End of string reached with no escape
                                (false, false)
                            } else {
                                (true, false)
                            }
                        });

                        cursor.next();

                        TokenKind::Literal {
                            kind: LiteralKind::String,
                            chars,
                        }
                    }
                    _ => TokenKind::Unknown,
                },
                position,
            )
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::lexer::cursor::Position;

    use super::*;

    fn tokenize(source: &str) -> Vec<Token> {
        super::tokenize(source).collect()
    }

    #[test]
    fn integer_token() {
        assert_eq!(
            tokenize("-90"),
            vec![
                Token {
                    kind: TokenKind::Minus,
                    position: Position::new()
                },
                Token {
                    kind: TokenKind::Literal {
                        kind: LiteralKind::Integer,
                        chars: vec!['9', '0']
                    },
                    position: Position::new()
                }
            ]
        );
    }
}
