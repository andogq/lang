use crate::{
    token::{Keyword, TokenKind},
    token_stream::{TokenIterator, TokenStream},
};

use self::{
    error::{ParserError, ParserResult},
    parsers::{Expression, Let},
};

pub mod error;
mod parsers;

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
