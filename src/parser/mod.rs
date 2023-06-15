use crate::token::{Keyword, Token, TokenKind};
use std::{iter::Peekable, vec::IntoIter};

use self::{
    _let::Let,
    error::{ParserError, ParserResult},
    expression::Expression,
};

mod _let;
mod error;
mod expression;

type PeekableTokens = Peekable<IntoIter<Token>>;

#[derive(Debug)]
pub enum AstNode {
    Let(Let),
    Expression(Expression),
}

pub fn parse(tokens: Vec<Token>) -> ParserResult<Vec<AstNode>> {
    let mut tokens = tokens.into_iter().peekable();
    let mut nodes = Vec::new();

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Keyword(Keyword::Let) => nodes.push(AstNode::Let(Let::parse(&mut tokens)?)),
            TokenKind::Comment(_) => (),
            t => {
                return Err(ParserError::UnexpectedToken {
                    token: t,
                    position: token.position,
                })
            }
        }
    }

    Ok(nodes)
}
