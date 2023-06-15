use crate::token::{LiteralKind, TokenKind};

use super::PeekableTokens;

/// Each of the binary operations that can take place within an expression.
#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperationKind {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mult,
    /// Division
    Div,
    /// Exponent
    Exp,
}

/// Each of the unary operations that can take place within an expression.
#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperationKind {
    /// Negation (eg `-8`)
    Negative,
}

/// Each of the possible expression types.
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    /// A single number. Eg `8`.
    Number(usize),
    /// A variable. Eg `a`.
    Ident(String),
    /// A binary operation. Eg `a + 8`.
    BinaryOperation {
        operation: BinaryOperationKind,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// A unary operation. Eg `-8`.
    UnaryOperation {
        operation: UnaryOperationKind,
        rhs: Box<Expression>,
    },
}

/// The following grammar is used to parse expressions. Expressions can be terminated by a number,
/// function call, or another variable.
///
/// ```txt
/// S -> E end
/// E -> T {("+" | "-") T}
/// T -> F {("*" | "/") F}
/// F -> P ["^" F]
/// P -> v | "(" E ")" | "-" T
/// v -> [0-9]+ | function | variable
/// ```
impl Expression {
    /// Parse the `E` term from the grammar
    /// ```txt
    /// E -> T {("+" | "-") T}
    /// ```
    pub fn parse_expression(tokens: &mut PeekableTokens) -> Expression {
        let mut expr = Self::parse_term(tokens);

        loop {
            let operation = tokens.peek().and_then(|t| match t.kind {
                TokenKind::Plus => Some(BinaryOperationKind::Add),
                TokenKind::Minus => Some(BinaryOperationKind::Sub),
                _ => None,
            });
            let Some(operation) = operation else { break };

            // Consume peeked token
            tokens.next();

            expr = Expression::BinaryOperation {
                operation,
                lhs: Box::new(expr),
                rhs: Box::new(Self::parse_term(tokens)),
            };
        }

        expr
    }

    /// Parse the `T` term from the grammar
    /// ```txt
    /// T -> F {("*" | "/") F}
    /// ```
    pub fn parse_term(tokens: &mut PeekableTokens) -> Expression {
        let mut expr = Self::parse_factor(tokens);

        loop {
            let operation = tokens.peek().and_then(|t| match t.kind {
                TokenKind::Asterix => Some(BinaryOperationKind::Mult),
                TokenKind::Slash => Some(BinaryOperationKind::Div),
                _ => None,
            });
            let Some(operation) = operation else { break };

            // Consume peeked token
            tokens.next();

            expr = Expression::BinaryOperation {
                operation,
                lhs: Box::new(expr),
                rhs: Box::new(Self::parse_factor(tokens)),
            };
        }

        expr
    }

    /// Parse the `F` term from the grammar
    /// ```txt
    /// F -> P ["^" F]
    /// ```
    pub fn parse_factor(tokens: &mut PeekableTokens) -> Expression {
        let p = Self::parse_primary(tokens);

        if tokens
            .peek()
            .map(|t| matches!(t.kind, TokenKind::Hat))
            .unwrap_or_default()
        {
            // Consume hat
            tokens.next();

            Expression::BinaryOperation {
                operation: BinaryOperationKind::Exp,
                lhs: Box::new(p),
                rhs: Box::new(Self::parse_factor(tokens)),
            }
        } else {
            p
        }
    }

    /// Parse the `P` term from the grammar
    /// ```txt
    /// P -> v | "(" E ")" | "-" T
    /// ```
    pub fn parse_primary(tokens: &mut PeekableTokens) -> Expression {
        match tokens.next().expect("token to follow").kind {
            TokenKind::Literal {
                kind: LiteralKind::Integer,
                chars,
            } => Expression::Number(
                dbg!(chars)
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("valid base 10 number"),
            ),
            TokenKind::Identifier(ident) => Expression::Ident(ident),
            TokenKind::LSmooth => {
                let expression = Self::parse_expression(tokens);
                let Some(TokenKind::RSmooth) = tokens.next().map(|t| t.kind) else {
                    panic!("expected RSmooth");
                };
                expression
            }
            TokenKind::Minus => Expression::UnaryOperation {
                operation: UnaryOperationKind::Negative,
                rhs: Box::new(Self::parse_term(tokens)),
            },
            _ => panic!("unexpected token"),
        }
    }

    /// Parses tokens into an expression (identical to [Self::parse_expression] call).
    pub fn parse(tokens: &mut PeekableTokens) -> Expression {
        Self::parse_expression(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn number_expressions() {
        assert_eq!(
            Expression::parse(
                &mut vec![Token {
                    kind: TokenKind::Literal {
                        kind: crate::token::LiteralKind::Integer,
                        chars: vec!['9', '0'],
                    },
                }]
                .into_iter()
                .peekable(),
            ),
            Expression::Number(90)
        );

        assert_eq!(
            Expression::parse(
                &mut vec![Token {
                    kind: TokenKind::Literal {
                        kind: crate::token::LiteralKind::Integer,
                        chars: vec!['0', '0', '0', '0', '9', '0'],
                    },
                }]
                .into_iter()
                .peekable(),
            ),
            Expression::Number(90)
        );
    }

    #[test]
    fn unary_operation() {
        assert_eq!(
            Expression::parse(
                &mut vec![
                    Token {
                        kind: TokenKind::Minus,
                    },
                    Token {
                        kind: TokenKind::Literal {
                            kind: crate::token::LiteralKind::Integer,
                            chars: vec!['9', '0'],
                        },
                    }
                ]
                .into_iter()
                .peekable(),
            ),
            Expression::UnaryOperation {
                operation: UnaryOperationKind::Negative,
                rhs: Box::new(Expression::Number(90))
            }
        );

        assert_eq!(
            Expression::parse(
                &mut vec![
                    Token {
                        kind: TokenKind::Minus,
                    },
                    Token {
                        kind: TokenKind::Minus,
                    },
                    Token {
                        kind: TokenKind::Literal {
                            kind: crate::token::LiteralKind::Integer,
                            chars: vec!['9', '0'],
                        },
                    }
                ]
                .into_iter()
                .peekable(),
            ),
            Expression::UnaryOperation {
                operation: UnaryOperationKind::Negative,
                rhs: Box::new(Expression::UnaryOperation {
                    operation: UnaryOperationKind::Negative,
                    rhs: Box::new(Expression::Number(90))
                })
            }
        );
    }
}
