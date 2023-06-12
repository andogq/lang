use crate::{
    lexer::cursor::Cursor,
    token::{LiteralKind, Token, TokenKind},
};

mod cursor;

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(source);

    std::iter::from_fn(move || {
        cursor.next().map(|c| {
            Token::new(match c {
                '=' => TokenKind::Equals,
                '+' => TokenKind::Plus,
                ';' => TokenKind::Semi,
                '/' if cursor.peek_next().map(|c| c == '/').unwrap_or_default() => {
                    // Skip next `/`
                    cursor.next();

                    TokenKind::Comment(String::from_iter(cursor.take_while(|c| c != '\n')))
                }
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
                    TokenKind::Identifier(String::from_iter(cursor.retake_while(is_ident_char)))
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
            })
        })
    })
}
