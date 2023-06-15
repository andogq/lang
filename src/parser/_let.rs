use crate::{token::TokenKind, token_stream::TokenIterator};

use super::{
    error::{ParserError, ParserResult},
    expression::Expression,
    TokenStream,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Let {
    ident: String,
    rhs: Expression,
}
impl Let {
    pub fn parse<I>(tokens: &mut TokenStream<I>) -> ParserResult<Let>
    where
        I: TokenIterator,
    {
        let token = tokens.next()?;
        let TokenKind::Identifier(ident) = token.kind else {
            return Err(ParserError::ExpectedToken { token: TokenKind::Identifier(String::new()), position: token.position })
        };

        tokens.expect(TokenKind::Equals)?;

        let expression = Expression::parse(tokens)?;

        tokens.expect(TokenKind::Semi)?;

        Ok(Let {
            ident,
            rhs: expression,
        })
    }
}
