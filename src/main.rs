use new_compiler::lexer::lex::tokenize;

fn main() {
    let tokens = tokenize("let x = 10;".to_string()).unwrap();
    for token in tokens {
        println!("{:?}", token);
    }
}
