use crate::token::TokenKind;

use super::{expression::Expression, PeekableTokens};

#[derive(Debug)]
pub struct Let {
    ident: String,
    rhs: Expression,
}
impl Let {
    pub fn parse(tokens: &mut PeekableTokens) -> Let {
        let Some(TokenKind::Identifier(ident)) = tokens.next().map(|t| t.kind) else {
        panic!("ident token following let");
    };

        // Consume = sign
        let Some(TokenKind::Equals) = tokens.next().map(|t| t.kind) else {
        panic!("equals token following let ident");
    };

        let expression = Expression::parse(tokens);

        let Some(TokenKind::Semi) = tokens.next().map(|t| t.kind) else {
        panic!("expected semi colon following let statement");
    };

        Let {
            ident,
            rhs: expression,
        }
    }
}
