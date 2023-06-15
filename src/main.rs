use checks::typing::{TypeEnvironment, TypeError};
use parser::error::ParserError;
use thiserror::Error;
use token_stream::TokenStream;

use crate::{lexer::Lexer, parser::parse, token::TokenKind};

mod checks;
mod lexer;
mod parser;
mod token;
mod token_stream;

#[derive(Debug, Error)]
#[error(transparent)]
enum CompilerError {
    ParserError(#[from] ParserError),
    TypeError(#[from] TypeError),
}

fn main() -> Result<(), CompilerError> {
    let source = r#"let a = 3;
let b = 5;

// The result
let c = a + b;
"#;

    let tokens = Lexer::new(source).filter(|token| !matches!(token.kind, TokenKind::Whitespace));
    let ast = parse(TokenStream::from(tokens))?;
    let type_environment = TypeEnvironment::from_ast(ast)?;

    Ok(())
}
