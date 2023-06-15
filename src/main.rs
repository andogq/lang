use parser::error::ParserResult;
use token_stream::TokenStream;

use crate::{lexer::Lexer, parser::parse, token::TokenKind};

mod lexer;
mod parser;
mod token;
mod token_stream;

fn main() -> ParserResult<()> {
    let source = r#"let a = 3;
let b = 5;

// The result
let c = a + b;

let a_string = "this is a \\very\\ cool \"string\"\\";
"#;

    let tokens = Lexer::new(source).filter(|token| !matches!(token.kind, TokenKind::Whitespace));
    let ast = parse(TokenStream::from(tokens));
    dbg!(ast)?;

    Ok(())
}
