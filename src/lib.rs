pub mod lexer;
pub mod macros;
pub mod errors;

#[derive(Debug, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}
