use crate::token::TokenKind;

use super::{
    error::{ParserError, ParserResult},
    expression::Expression,
    PeekableTokens,
};

#[derive(Debug)]
pub struct Let {
    ident: String,
    rhs: Expression,
}
impl Let {
    pub fn parse(tokens: &mut PeekableTokens) -> ParserResult<Let> {
        let token = tokens.next().ok_or(ParserError::ExpectedTokenToFollow)?;
        let TokenKind::Identifier(ident) = token.kind else {
            return Err(ParserError::ExpectedToken { token: TokenKind::Identifier(String::new()), position: token.position })
        };

        // Consume = sign
        let token = tokens.next().ok_or(ParserError::ExpectedTokenToFollow)?;
        let TokenKind::Equals = token.kind else {
            return Err(ParserError::ExpectedToken { token: TokenKind::Equals, position: token.position })
        };

        let expression = Expression::parse(tokens)?;

        let token = tokens.next().ok_or(ParserError::ExpectedTokenToFollow)?;
        let TokenKind::Semi = token.kind else {
            return Err(ParserError::ExpectedToken { token: TokenKind::Semi, position: token.position });
        };

        Ok(Let {
            ident,
            rhs: expression,
        })
    }
}
