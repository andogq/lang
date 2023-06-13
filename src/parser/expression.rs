use crate::token::TokenKind;

use super::PeekableTokens;

#[derive(Debug)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Mult,
    Div,
    Exp,
}

#[derive(Debug)]
pub enum Expression {
    Number(usize),
    Ident(String),
    BinaryOperation {
        operation: BinaryOperation,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

impl Expression {
    /*
    S -> E end
    E -> T {("+" | "-") T}
    T -> F {("*" | "/") F}
    F -> P ["^" F]
    P -> v | "(" E ")" | "-" T
    v -> [0-9]+ | function | variable
     */
    pub fn parse(tokens: &mut PeekableTokens) -> Expression {
        fn E(tokens: &mut PeekableTokens) -> Expression {
            let mut expr = T(tokens);

            loop {
                let operation = tokens.peek().and_then(|t| match t.kind {
                    TokenKind::Plus => Some(BinaryOperation::Plus),
                    TokenKind::Minus => Some(BinaryOperation::Minus),
                    _ => None,
                });
                let Some(operation) = operation else { break };

                // Consume peeked token
                tokens.next();

                expr = Expression::BinaryOperation {
                    operation,
                    lhs: Box::new(expr),
                    rhs: Box::new(T(tokens)),
                };
            }

            expr
        }

        fn T(tokens: &mut PeekableTokens) -> Expression {
            let mut expr = F(tokens);

            loop {
                let operation = tokens.peek().and_then(|t| match t.kind {
                    TokenKind::Asterix => Some(BinaryOperation::Mult),
                    TokenKind::Slash => Some(BinaryOperation::Div),
                    _ => None,
                });
                let Some(operation) = operation else { break };

                // Consume peeked token
                tokens.next();

                expr = Expression::BinaryOperation {
                    operation,
                    lhs: Box::new(expr),
                    rhs: Box::new(F(tokens)),
                };
            }

            expr
        }

        fn F(tokens: &mut PeekableTokens) -> Expression {
            let p = P(tokens);

            if tokens
                .peek()
                .map(|t| matches!(t.kind, TokenKind::Hat))
                .unwrap_or_default()
            {
                // Consume hat
                tokens.next();

                Expression::BinaryOperation {
                    operation: BinaryOperation::Exp,
                    lhs: Box::new(p),
                    rhs: Box::new(F(tokens)),
                }
            } else {
                p
            }
        }

        fn P(tokens: &mut PeekableTokens) -> Expression {
            match tokens.next().expect("token to follow").kind {
                TokenKind::Literal { .. } => Expression::Number(0), // TODO: parse number somewhere
                TokenKind::Identifier(ident) => Expression::Ident(ident),
                TokenKind::LSmooth => {
                    let expression = E(tokens);
                    let Some(TokenKind::RSmooth) = tokens.next().map(|t| t.kind) else {
                    panic!("expected RSmooth");
                };
                    expression
                }
                TokenKind::Minus => T(tokens),
                _ => panic!("unexpected token"),
            }
        }

        E(tokens)
    }
}
