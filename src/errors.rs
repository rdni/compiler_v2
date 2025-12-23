use std::fmt::Display;

use thiserror::Error;

use crate::Span;

#[derive(Debug, Clone)]
pub struct Error {
    internal_error: ErrorImpl,
    span: Span
}

impl Error {
    pub fn new(error_impl: ErrorImpl, span: Span) -> Self {
        Error {
            internal_error: error_impl,
            span
        }
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }

    pub fn get_error_name(&self) -> &str {
        match &self.internal_error {
            ErrorImpl::UnrecognisedToken { .. } => "UnrecognisedToken",
            _ => todo!()
        }
    }

    pub fn get_tip(&self) -> ErrorTip {
        match &self.internal_error {
            ErrorImpl::UnrecognisedToken { .. } => ErrorTip::Suggestion("Check for typos or unsupported characters.".to_string()),
            _ => todo!()
        }
    }
}

pub enum ErrorTip {
    None,
    Suggestion(String)
}

impl Display for ErrorTip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorTip::None => write!(f, ""),
            ErrorTip::Suggestion(suggestion) => write!(f, "{}", suggestion)
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ErrorImpl {
    #[error("unrecognised token: {token:?}")]
    UnrecognisedToken {
        token: String
    },
}