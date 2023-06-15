use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::{
    parser::error::{ParserError, ParserResult},
    token::{Token, TokenKind},
};

/// Used as a trait alias.
pub trait TokenIterator: Iterator<Item = Token> {}
impl<I> TokenIterator for I where I: Iterator<Item = Token> {}

pub struct TokenStream<I>(Peekable<I>)
where
    I: TokenIterator;

impl<I> Deref for TokenStream<I>
where
    I: TokenIterator,
{
    type Target = Peekable<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for TokenStream<I>
where
    I: TokenIterator,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I> TokenStream<I>
where
    I: TokenIterator,
{
    /// Consumes and returns the next token from the iterator, returning a
    /// [ParserError::ExpectedTokenToFollow] error if the next item is [None].
    pub fn next(&mut self) -> ParserResult<Token> {
        self.0.next().ok_or(ParserError::ExpectedTokenToFollow)
    }

    /// Peeks the next token in the stream, consuming it if it matches `token`, otherwise returns a
    /// [ParserError].
    pub fn expect(&mut self, token: TokenKind) -> ParserResult<Token> {
        let next_token = self.peek().ok_or(ParserError::ExpectedTokenToFollow)?;

        if next_token.kind == token {
            self.next()
        } else {
            Err(ParserError::ExpectedToken {
                token: next_token.kind.clone(),
                position: next_token.position.clone(),
            })
        }
    }
}

impl<I> From<Peekable<I>> for TokenStream<I>
where
    I: TokenIterator,
{
    fn from(iter: Peekable<I>) -> Self {
        Self(iter)
    }
}

impl<I> From<I> for TokenStream<I>
where
    I: TokenIterator,
{
    fn from(iter: I) -> Self {
        Self(iter.peekable())
    }
}
