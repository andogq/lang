use crate::{
    lexer::cursor::Cursor,
    token::{Keyword, LiteralKind, Token, TokenKind},
};

use self::cursor::TakeOption;

pub mod cursor;

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next().map(|(c, position)| {
            Token::new(
                match c {
                    '=' => TokenKind::Equals,
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Asterix,
                    '^' => TokenKind::Hat,
                    ';' => TokenKind::Semi,
                    '/' if self
                        .cursor
                        .peek_next()
                        .map(|c| c == '/')
                        .unwrap_or_default() =>
                    {
                        // Skip next `/`
                        self.cursor.next();

                        TokenKind::Comment(String::from_iter(self.cursor.take_while(|c| c != '\n')))
                    }
                    '/' => TokenKind::Slash,
                    '(' => TokenKind::LSmooth,
                    ')' => TokenKind::RSmooth,
                    c if c.is_ascii_whitespace() => {
                        // Consume through to the end of whitespace
                        self.cursor.skip_while(|c| c.is_ascii_whitespace());

                        TokenKind::Whitespace
                    }
                    c if c.is_ascii_digit() => TokenKind::Literal {
                        kind: LiteralKind::Integer,
                        chars: self.cursor.retake_while(|c| c.is_ascii_digit()),
                    },
                    c if is_ident_char(c) => {
                        let ident_str = String::from_iter(self.cursor.retake_while(is_ident_char));
                        match ident_str.as_str() {
                            // Match for keywords
                            "let" => TokenKind::Keyword(Keyword::Let),
                            _ => TokenKind::Identifier(ident_str),
                        }
                    }
                    '"' => {
                        let chars = self.cursor.take_while_config(false, false, |c, escaped| {
                            if c == '\\' {
                                if !escaped {
                                    // Don't emit this back slash, as it is causing an escape
                                    (TakeOption::Skip, true)
                                } else {
                                    // Backslash can escape itself
                                    (TakeOption::Take, false)
                                }
                            } else if c == '"' && !escaped {
                                // End of string reached with no escape
                                (TakeOption::SkipAndStop, false)
                            } else {
                                (TakeOption::Take, false)
                            }
                        });

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
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::cursor::Position;

    use super::*;

    #[test]
    fn integer() {
        assert_eq!(
            Lexer::new("-90").collect::<Vec<_>>(),
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

    #[test]
    fn string() {
        assert_eq!(
            Lexer::new(r#""this is a \\very\\ cool \"string\"\\""#).collect::<Vec<_>>(),
            vec![Token {
                kind: TokenKind::Literal {
                    kind: LiteralKind::String,
                    chars: vec![
                        't', 'h', 'i', 's', ' ', 'i', 's', ' ', 'a', ' ', '\\', 'v', 'e', 'r', 'y',
                        '\\', ' ', 'c', 'o', 'o', 'l', ' ', '"', 's', 't', 'r', 'i', 'n', 'g', '"',
                        '\\'
                    ]
                },
                position: Position::new()
            }]
        )
    }
}
