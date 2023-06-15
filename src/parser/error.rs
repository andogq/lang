use thiserror::Error;

use crate::{lexer::cursor::Position, token::TokenKind};

/// A result type for use within the parser.
pub type ParserResult<T> = Result<T, ParserError>;

/// All the possible errors that could be encountered whilst attempting to parse.
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("expected token to follow, but found none")]
    ExpectedTokenToFollow,
    #[error("{position}: expected {token:?}")]
    ExpectedToken {
        token: TokenKind,
        position: Position,
    },
    #[error("{position}: unexpected {token:?}")]
    UnexpectedToken {
        token: TokenKind,
        position: Position,
    },
}
