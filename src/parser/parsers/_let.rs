use crate::{token::TokenKind, token_stream::TokenIterator};

use super::{
    super::{
        error::{ParserError, ParserResult},
        TokenStream,
    },
    Expression,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Let {
    pub(crate) ident: String,
    pub(crate) rhs: Expression,
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
