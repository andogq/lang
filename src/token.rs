#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
}
impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
pub enum LiteralKind {
    String,
    Integer,
}

#[derive(Debug)]
pub enum TokenKind {
    Literal { kind: LiteralKind, chars: Vec<char> },
    Identifier(String),
    Whitespace,
    Semi,
    Comment(String),

    Equals,
    Plus,

    Unknown,
}
