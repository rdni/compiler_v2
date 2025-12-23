#[macro_export]
macro_rules! MK_DEFAULT_HANDLER {
    ($kind:expr, $value:literal) => {{
        use $crate::Span;
        |lexer: &mut Lexer, _regex: Regex| {
            lexer.push(MK_TOKEN!(
                $kind,
                String::from($value),
                Span {
                    start: lexer.position,
                    end: lexer.position + $value.len()
                }
            ));
            lexer.advance_n($value.len().try_into().unwrap());
        }
    }};
}

#[macro_export]
macro_rules! MK_TOKEN {
    ($kind:expr, $lexeme:expr, $span:expr) => {
        Token {
            kind: $kind,
            lexeme: $lexeme,
            span: $span,
        }
    };
}
