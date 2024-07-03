use core::fmt;
use std::string::ParseError;

use wasm_bindgen::{JsError, JsValue};

use crate::lexer::enums::token::Token;

#[derive(Debug)]
pub enum PseudoError {
    IoError(std::io::Error),
    JsError(JsValue),
    ParseError(String),
    KeywordError(String),
    AssignmentError(String),
    StatementError(String),
    InvalidOperation,
    ValueError(String),
    InvalidToken(String),
    EvalError(String),
    VariableError(String),
    TypeError(String),
    RuntimeError(String),
    UnexpectedEOF,
}

impl fmt::Display for PseudoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PseudoError::IoError(err) => write!(f, "IO Error: {}", err),
            PseudoError::JsError(err) => write!(f, "JavaScript Error: {:?}", err),
            PseudoError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            PseudoError::KeywordError(msg) => write!(f, "Keyword Error: {}", msg),
            PseudoError::AssignmentError(msg) => write!(f, "Assignment Error: {}", msg),
            PseudoError::StatementError(msg) => write!(f, "Statement Error: {}", msg),
            PseudoError::InvalidOperation => write!(f, "Invalid Operation"),
            PseudoError::ValueError(msg) => write!(f, "Value Error: {}", msg),
            PseudoError::InvalidToken(msg) => write!(f, "Invalid Token: {}", msg),
            PseudoError::EvalError(msg) => write!(f, "Evaluation Error: {}", msg),
            PseudoError::VariableError(msg) => write!(f, "Variable Error: {}", msg),
            PseudoError::TypeError(msg) => write!(f, "Type Error: {}", msg),
            PseudoError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
            PseudoError::UnexpectedEOF => write!(f, "Unexpected End of File"),
        }
    }
}

impl From<PseudoError> for JsError {
    fn from(value: PseudoError) -> Self {
        JsError::new(&value.to_string())
    }
}

impl From<std::io::Error> for PseudoError {
    fn from(err: std::io::Error) -> Self {
        PseudoError::IoError(err)
    }
}

impl From<ParseError> for PseudoError {
    fn from(err: ParseError) -> Self {
        PseudoError::ParseError(err.to_string())
    }
}

pub trait KeywordError {
    fn keyword(expected: Vec<Token>, found: &Token) -> Self;
}

// expected keyword error
impl KeywordError for PseudoError {
    fn keyword(expected: Vec<Token>, found: &Token) -> Self {
        PseudoError::KeywordError(format!("Expected: '{}', found: '{}'", expected[0], found))
    }
}
