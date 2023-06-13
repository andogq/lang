use std::{iter::Peekable, vec::IntoIter};

use crate::token::{Keyword, Token, TokenKind};

type PeekableTokens = Peekable<IntoIter<Token>>;

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

#[derive(Debug)]
pub enum AstNode {
    Let { ident: String, rhs: Expression },
    Expression(Expression),
}

/*
S -> E end
E -> T {("+" | "-") T}
T -> F {("*" | "/") F}
F -> P ["^" F]
P -> v | "(" E ")" | "-" T
v -> [0-9]+ | function | variable
 */
pub fn parse_expression(tokens: &mut PeekableTokens) -> Expression {
    fn E(tokens: &mut PeekableTokens) -> Expression {
        let mut expr = T(tokens);

        while let Some(TokenKind::Plus | TokenKind::Minus) = tokens.peek().map(|t| t.kind.clone()) {
            let op = tokens.next().unwrap().kind;
            expr = Expression::BinaryOperation {
                operation: match op {
                    TokenKind::Plus => BinaryOperation::Plus,
                    TokenKind::Minus => BinaryOperation::Minus,
                    _ => unreachable!(),
                },
                lhs: Box::new(expr),
                rhs: Box::new(T(tokens)),
            };
        }

        expr
    }

    fn T(tokens: &mut PeekableTokens) -> Expression {
        let mut expr = F(tokens);

        while let Some(TokenKind::Asterix | TokenKind::Slash) =
            tokens.peek().map(|t| t.kind.clone())
        {
            let op = tokens.next().unwrap().kind;
            expr = Expression::BinaryOperation {
                operation: match op {
                    TokenKind::Asterix => BinaryOperation::Mult,
                    TokenKind::Slash => BinaryOperation::Div,
                    _ => unreachable!(),
                },
                lhs: Box::new(expr),
                rhs: Box::new(F(tokens)),
            };
        }

        expr
    }

    fn F(tokens: &mut PeekableTokens) -> Expression {
        let p = P(tokens);

        if let Some(TokenKind::Hat) = tokens.peek().map(|t| t.kind.clone()) {
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

pub fn parse_let(tokens: &mut PeekableTokens) -> AstNode {
    let Some(TokenKind::Identifier(ident)) = tokens.next().map(|t| t.kind) else {
        panic!("ident token following let");
    };

    // Consume = sign
    let Some(TokenKind::Equals) = tokens.next().map(|t| t.kind) else {
        panic!("equals token following let ident");
    };

    let expression = parse_expression(tokens);

    let Some(TokenKind::Semi) = tokens.next().map(|t| t.kind) else {
        panic!("expected semi colon following let statement");
    };

    AstNode::Let {
        ident,
        rhs: expression,
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut tokens = tokens.into_iter().peekable();
    let mut nodes = Vec::new();

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Keyword(Keyword::Let) => nodes.push(parse_let(&mut tokens)),
            TokenKind::Comment(_) => (),
            _ => panic!("unexpected token {:?}", token.kind),
        }
    }

    nodes
}
