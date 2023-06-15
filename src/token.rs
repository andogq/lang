#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
}
impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
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
