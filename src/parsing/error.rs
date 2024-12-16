use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingErrorType {
    PeekError,
    DataError,
}

#[derive(Debug)]
pub struct ParsingError {
    error_type: ParsingErrorType,
    message: String,
}

impl ParsingError {
    pub fn new(error_type: ParsingErrorType, message: String) -> Self {
        ParsingError {
            error_type,
            message,
        }
    }

    pub fn get_type(&self) -> ParsingErrorType {
        self.error_type.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.get_type() {
            ParsingErrorType::PeekError => write!(f, "Peek Error: {}.", self.get_message()),
            _ => write!(f, "Unknown Error: {}.", self.message),
        }
    }
}
impl Error for ParsingError {}
