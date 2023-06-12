use lexer::tokenize;

mod lexer;
mod token;

fn main() {
    let source = r#"let a = 3;
let b = 5;

// The result
let c = a + b;

let a_string = "this is a \\very\\ cool \"string\"\\";
"#;

    let tokens = tokenize(source).collect::<Vec<_>>();

    dbg!(tokens);
}
