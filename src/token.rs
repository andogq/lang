use crate::lexer::cursor::Position;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}
impl Token {
    pub fn new(kind: TokenKind, position: Position) -> Self {
        Self { kind, position }
    }
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralKind {
    String,
    Integer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Let,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Literal { kind: LiteralKind, chars: Vec<char> },
    Identifier(String),
    Whitespace,
    Semi,
    Comment(String),

    Equals,
    Plus,
    Minus,
    Asterix,
    Slash,
    Hat,

    LSmooth,
    RSmooth,

    Unknown,
}
