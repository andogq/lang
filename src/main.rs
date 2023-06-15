use checks::typing::{TypeEnvironment, TypeError};
use lexer::LexerError;
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
    LexerError(#[from] LexerError),
    ParserError(#[from] ParserError),
    TypeError(#[from] TypeError),
}

fn main() -> Result<(), CompilerError> {
    let source = r#"let a = 3;
let b = 5;

// The result
let c = a + b;

let some_bool = true;
let another_bool = false;
"#;

    let tokens = Lexer::new(source)
        .filter(|token| {
            token
                .as_ref()
                .map(|token| !matches!(token.kind, TokenKind::Whitespace))
                .unwrap_or_default()
        })
        // Don't like that the iterator is consumed here just to get the errors out
        .collect::<Result<Vec<_>, _>>()?;

    let ast = parse(TokenStream::from(tokens.into_iter()))?;
    let type_environment = TypeEnvironment::from_ast(ast)?;

    Ok(())
}
