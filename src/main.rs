use lexer::tokenize;

use crate::{parser::parse, token::TokenKind};

mod lexer;
mod parser;
mod token;

fn main() {
    let source = r#"let a = 3;
let b = 5;

// The result
let c = a + b;

let a_string = "this is a \\very\\ cool \"string\"\\";
"#;

    let tokens = tokenize(source)
        .filter(|token| !matches!(token.kind, TokenKind::Whitespace))
        .collect::<Vec<_>>();
    dbg!(&tokens);

    let ast = parse(tokens);
    dbg!(ast);
}
