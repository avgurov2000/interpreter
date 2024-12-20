use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ExpressionErrorType {
    IdentifierError,
    IntegerLiteralError,
    PrefixError,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum StatementErrorType {
    ExpressionError,
    LetError,
    ReturnError,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ASTErrorType {
    ExpressionError(ExpressionErrorType),
    StatementError(StatementErrorType),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ASTError {
    error_type: ASTErrorType,
    message: String,
}

impl ASTError {
    pub fn new(error_type: ASTErrorType, message: String) -> Self {
        ASTError {
            error_type,
            message,
        }
    }

    pub fn get_type(&self) -> ASTErrorType {
        self.error_type.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl Display for ASTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AST Error: {}.", self.message)
    }
}
impl Error for ASTError {}
