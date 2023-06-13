#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}
impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    String,
    Integer,
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Let,
}

#[derive(Debug, Clone)]
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
