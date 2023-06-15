use crate::{
    token::{Keyword, TokenKind},
    token_stream::{TokenIterator, TokenStream},
};

use self::{
    _let::Let,
    error::{ParserError, ParserResult},
    expression::Expression,
};

mod _let;
pub mod error;
mod expression;

#[allow(unused)]
#[derive(Debug)]
pub enum AstNode {
    Let(Let),
    Expression(Expression),
}

pub fn parse<I>(mut tokens: TokenStream<I>) -> ParserResult<Vec<AstNode>>
where
    I: TokenIterator,
{
    let mut nodes = Vec::new();

    while let Ok(token) = tokens.next() {
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
