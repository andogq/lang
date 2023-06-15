use std::collections::{hash_map::Entry, HashMap};

use thiserror::Error;

use crate::{
    parser::{parsers::Expression, AstNode},
    token::LiteralKind,
};

// Each of the possible types that can be expressed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Integer,
    String,
}

/// All of the possible type errors that could arise through type checking
#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Unknown ident {0}")]
    UnknownIdent(String),
    #[error("Cannot redeclare ident {0} (yet)")]
    IdentRedeclared(String),
    #[error("Mismatched types: {lhs:?} and {rhs:?}")]
    MismatchedTypes { lhs: Type, rhs: Type },
}

#[derive(Default)]
pub struct TypeEnvironment {
    ident_types: HashMap<String, Type>,
}
impl TypeEnvironment {
    /// Creates a typed environment from an AST.
    pub fn from_ast(ast: Vec<AstNode>) -> Result<Self, TypeError> {
        let mut environment = Self::default();

        for node in ast {
            match node {
                AstNode::Let(let_node) => {
                    // Determine type of expression
                    let expression_type = environment.get_expression_type(let_node.rhs)?;

                    // Add type of expression to environment hashmap
                    match environment.ident_types.entry(let_node.ident.clone()) {
                        Entry::Vacant(entry) => entry.insert(expression_type),
                        Entry::Occupied(_) => {
                            return Err(TypeError::IdentRedeclared(let_node.ident))
                        }
                    };
                }
                AstNode::Expression(expression_node) => {
                    // Validate type of expression
                    environment.get_expression_type(expression_node)?;
                }
            }
        }

        Ok(environment)
    }

    /// Determines the type of an expression
    fn get_expression_type(&self, expression: Expression) -> Result<Type, TypeError> {
        match expression {
            Expression::Ident(ident) => self
                .ident_types
                .get(&ident)
                .ok_or(TypeError::UnknownIdent(ident))
                .copied(),
            Expression::BinaryOperation { lhs, rhs, .. } => {
                // Check if lhs and rhs have compatible types
                let lhs_type = self.get_expression_type(*lhs)?;
                let rhs_type = self.get_expression_type(*rhs)?;

                if lhs_type == rhs_type {
                    Ok(lhs_type)
                } else {
                    Err(TypeError::MismatchedTypes {
                        lhs: lhs_type,
                        rhs: rhs_type,
                    })
                }
            }
            Expression::UnaryOperation { rhs, .. } => {
                // TODO: Make sure operation can be applied to RHS
                self.get_expression_type(*rhs)
            }
            Expression::Literal { kind, .. } => Ok(match kind {
                LiteralKind::Integer => Type::Integer,
                LiteralKind::String => Type::String,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parsers::{BinaryOperationKind, Expression, Let};

    #[test]
    fn assignment() {
        assert_eq!(
            TypeEnvironment::from_ast(vec![AstNode::Let(Let {
                ident: "a".to_string(),
                rhs: Expression::Literal {
                    kind: LiteralKind::Integer,
                    chars: vec!['1', '0']
                }
            })])
            .unwrap()
            .ident_types,
            HashMap::from([("a".to_string(), Type::Integer)])
        );
    }

    #[test]
    fn duplicated_assignment() {
        assert!(matches!(
            TypeEnvironment::from_ast(vec![
                AstNode::Let(Let {
                    ident: "a".to_string(),
                    rhs: Expression::Literal {
                        kind: LiteralKind::Integer,
                        chars: vec!['1', '0']
                    }
                }),
                AstNode::Let(Let {
                    ident: "a".to_string(),
                    rhs: Expression::Literal {
                        kind: LiteralKind::Integer,
                        chars: vec!['1', '0']
                    }
                })
            ]),
            Err(TypeError::IdentRedeclared(_))
        ));
    }

    #[test]
    fn nested_assignment() {
        assert_eq!(
            TypeEnvironment::from_ast(vec![
                AstNode::Let(Let {
                    ident: "a".to_string(),
                    rhs: Expression::Literal {
                        kind: LiteralKind::Integer,
                        chars: vec!['1', '0']
                    }
                }),
                AstNode::Let(Let {
                    ident: "b".to_string(),
                    rhs: Expression::Literal {
                        kind: LiteralKind::Integer,
                        chars: vec!['1', '0']
                    }
                }),
                AstNode::Let(Let {
                    ident: "c".to_string(),
                    rhs: Expression::BinaryOperation {
                        operation: BinaryOperationKind::Add,
                        lhs: Box::new(Expression::Literal {
                            kind: LiteralKind::Integer,
                            chars: vec!['1', '0']
                        }),
                        rhs: Box::new(Expression::BinaryOperation {
                            operation: BinaryOperationKind::Mult,
                            lhs: Box::new(Expression::Ident("b".to_string())),
                            rhs: Box::new(Expression::Literal {
                                kind: LiteralKind::Integer,
                                chars: vec!['1', '0']
                            })
                        })
                    }
                })
            ])
            .unwrap()
            .ident_types,
            HashMap::from([
                ("a".to_string(), Type::Integer),
                ("b".to_string(), Type::Integer),
                ("c".to_string(), Type::Integer)
            ])
        );
    }
}
