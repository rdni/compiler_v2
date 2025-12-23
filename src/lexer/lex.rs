use regex::Regex;

use crate::{
    MK_DEFAULT_HANDLER, MK_TOKEN, Span, errors::{Error, ErrorImpl}, lexer::token::{RESERVED_LOOKUP, Token, TokenKind}
};

pub type RegexHandler = fn(&mut Lexer, Regex);

#[derive(Clone)]
pub struct RegexPattern {
    pub regex: Regex,
    pub handler: RegexHandler,
}

#[derive(Clone)]
pub struct Lexer {
    source: String,
    position: usize,
    regex_patterns: Vec<RegexPattern>,
    output: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            position: 0,
            regex_patterns: vec![
                RegexPattern {
                    regex: Regex::new("[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
                    handler: symbol_handler,
                },
                RegexPattern {
                    regex: Regex::new("[0-9]+(\\.[0-9]+)?").unwrap(),
                    handler: number_handler,
                },
                RegexPattern {
                    regex: Regex::new("\\s+").unwrap(),
                    handler: skip_handler,
                },
                RegexPattern {
                    regex: Regex::new("\"[^\"]*\"").unwrap(),
                    handler: string_handler,
                },
                RegexPattern {
                    regex: Regex::new("\\/\\/.*").unwrap(),
                    handler: skip_handler,
                },
                RegexPattern {
                    regex: Regex::new("\\[").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::OpenBracket, "["),
                },
                RegexPattern {
                    regex: Regex::new("\\]").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::CloseBracket, "]"),
                },
                RegexPattern {
                    regex: Regex::new("\\{").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::OpenCurly, "{"),
                },
                RegexPattern {
                    regex: Regex::new("\\}").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::CloseCurly, "}"),
                },
                RegexPattern {
                    regex: Regex::new("\\(").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::OpenParen, "("),
                },
                RegexPattern {
                    regex: Regex::new("\\)").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::CloseParen, ")"),
                },
                RegexPattern {
                    regex: Regex::new("==").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Equals, "=="),
                },
                RegexPattern {
                    regex: Regex::new("!=").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::NotEquals, "!="),
                },
                RegexPattern {
                    regex: Regex::new("!").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Not, "!"),
                },
                RegexPattern {
                    regex: Regex::new("=").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Assignment, "="),
                },
                RegexPattern {
                    regex: Regex::new("<=").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::LessEquals, "<="),
                },
                RegexPattern {
                    regex: Regex::new("<").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Less, "<"),
                },
                RegexPattern {
                    regex: Regex::new(">=").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::GreaterEquals, ">="),
                },
                RegexPattern {
                    regex: Regex::new(">").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Greater, ">"),
                },
                RegexPattern {
                    regex: Regex::new("\\|\\|").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Or, "||"),
                },
                RegexPattern {
                    regex: Regex::new("&&").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::And, "&&"),
                },
                // RegexPattern { regex: Regex::new("\\.\\.").unwrap(), handler: MK_DEFAULT_HANDLER!(TokenKind::DotDot, "..")},
                RegexPattern {
                    regex: Regex::new("\\.").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Dot, "."),
                },
                RegexPattern {
                    regex: Regex::new(";").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Semicolon, ";"),
                },
                RegexPattern {
                    regex: Regex::new(":").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Colon, ":"),
                },
                RegexPattern {
                    regex: Regex::new("\\?").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Question, "?"),
                },
                RegexPattern {
                    regex: Regex::new(",").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Comma, ","),
                },
                RegexPattern {
                    regex: Regex::new("\\+\\+").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::PlusPlus, "++"),
                },
                RegexPattern {
                    regex: Regex::new("->").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Arrow, "->"),
                },
                RegexPattern {
                    regex: Regex::new("--").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::MinusMinus, "--"),
                },
                RegexPattern {
                    regex: Regex::new("\\+=").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::PlusEquals, "+="),
                },
                RegexPattern {
                    regex: Regex::new("-=").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::MinusEquals, "-="),
                },
                RegexPattern {
                    regex: Regex::new("\\+").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Plus, "+"),
                },
                RegexPattern {
                    regex: Regex::new("-").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Dash, "-"),
                },
                RegexPattern {
                    regex: Regex::new("/").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Slash, "/"),
                },
                RegexPattern {
                    regex: Regex::new("\\*").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Star, "*"),
                },
                RegexPattern {
                    regex: Regex::new("%").unwrap(),
                    handler: MK_DEFAULT_HANDLER!(TokenKind::Percent, "%"),
                },
            ],
            output: Vec::new(),
        }
    }

    pub fn advance_n(&mut self, n: usize) {
        self.position += n;
    }

    pub fn is_at_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    pub fn at(&self) -> char {
        self.source.as_bytes()[self.position] as char
    }

    pub fn remainder(&self) -> Vec<char> {
        (self.source.as_bytes()[(self.position)..])
            .iter()
            .map(|x| *x as char)
            .collect::<Vec<char>>()
    }

    pub fn push(&mut self, token: Token) {
        self.output.push(token);
    }
}

fn number_handler(lexer: &mut Lexer, regex: Regex) {
    let remaining = &lexer.remainder().iter().collect::<String>();
    let matched = regex.find(remaining).unwrap().as_str().to_string();

    lexer.push(MK_TOKEN!(
        TokenKind::Number,
        matched.clone(),
        Span {
            start: lexer.position,
            end: lexer.position + matched.len()
        }
    ));
    lexer.advance_n(matched.len());
}

fn skip_handler(lexer: &mut Lexer, regex: Regex) {
    let remaining = &lexer.remainder().iter().collect::<String>();
    let matched = regex.find(remaining).unwrap().end();
    lexer.advance_n(matched);
}

fn string_handler(lexer: &mut Lexer, regex: Regex) {
    let binding = lexer.remainder().iter().collect::<String>();
    let matched = regex.find(&binding).unwrap();
    let mut string_literal = lexer.remainder()[(matched.start() + 1)..(matched.end() - 1)]
        .iter()
        .collect::<String>();

    lexer.advance_n(string_literal.len() + 2);

    let mut result = String::new();
    let mut chars = string_literal.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next_ch) = chars.peek() {
                match next_ch {
                    'n' => {
                        result.push('\n');
                        chars.next();
                    }
                    't' => {
                        result.push('\t');
                        chars.next();
                    }
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    'r' => {
                        result.push('\r');
                        chars.next();
                    }
                    '"' => {
                        result.push('"');
                        chars.next();
                    }
                    '0' => {
                        result.push('\0');
                        chars.next();
                    }
                    'x' => {
                        let mut hex = String::new();
                        chars.next();

                        for _ in 0..2 {
                            if let Some(ch) = chars.peek() {
                                if ch.is_ascii_hexdigit() {
                                    hex.push(*ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                        }

                        result.push(u8::from_str_radix(&hex, 16).unwrap() as char);
                    }
                    _ => {
                        result.push(ch); // Keep the backslash
                    }
                }
            } else {
                result.push(ch); // Keep the lone backslash
            }
        } else {
            result.push(ch); // Keep non-escape characters
        }
    }

    string_literal = result;

    lexer.push(MK_TOKEN!(
        TokenKind::String,
        string_literal.clone(),
        Span {
            start: lexer.position,
            end: lexer.position + string_literal.len()
        }
    ));
}

fn symbol_handler(lexer: &mut Lexer, regex: Regex) {
    let binding = lexer.remainder().iter().collect::<String>();
    let value = regex.find(&binding).unwrap();

    if let Some(kind) = RESERVED_LOOKUP.get(value.as_str()) {
        lexer.push(MK_TOKEN!(
            *kind,
            String::from(value.as_str()),
            Span {
                start: lexer.position,
                end: lexer.position + value.len()
            }
        ));
    } else {
        lexer.push(MK_TOKEN!(
            TokenKind::Identifier,
            String::from(value.as_str()),
            Span {
                start: lexer.position,
                end: lexer.position + value.len()
            }
        ));
    }

    lexer.advance_n(value.len());
}


pub fn tokenize(source: String) -> Result<Vec<Token>, Error> {
    let mut lex = Lexer::new(source);

    while !lex.is_at_eof() {
        let mut matched = false;
        
        for pattern in lex.clone().regex_patterns.iter() {
            let string = &lex.remainder().iter().collect::<String>();
            let match_here = pattern.regex.find(string);

            if match_here.is_some() && match_here.unwrap().start() == 0 {
                (pattern.handler)(&mut lex, pattern.regex.clone());
                matched = true;
                break;
            }
        }

        if !matched {
            return Err(Error::new(ErrorImpl::UnrecognisedToken { token: lex.at().to_string() }, Span {
                start: lex.position,
                end: lex.position + 1
            }));
        }
    }

    lex.push(MK_TOKEN!(TokenKind::EOF, String::from("EOF"), Span { start: lex.position, end: lex.position }));
    Ok(lex.output)
}