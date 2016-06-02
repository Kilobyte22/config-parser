use super::lexer::Token;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    UnexpectedEOF,
    Unexpected(Token),
    MissingParameter(String)
}

pub trait CodePos {
    fn location(&self) -> (u32, u16);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    error_type: ErrorType,
    line: u32,
    col: u16
}

impl Error {
    pub fn new(line: u32, col: u16, etype: ErrorType) -> Error {
        Error {
            error_type: etype,
            line: line,
            col: col
        }
    }

    pub fn from_state<T> (pos: &T, etype: ErrorType) -> Error where T: CodePos {
        let p = pos.location();
        Error::new(p.0, p.1, etype)
    }
}
