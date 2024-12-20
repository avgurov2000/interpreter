use std::any::Any;
use std::error::Error;
use std::fmt::Write;

use super::super::super::tokens::Token;
use super::super::error::{ASTError, ASTErrorType, StatementErrorType};
use super::super::{Expression, Node, Statement};
pub struct ExpressionStatement {
    token: Token,
    value: Option<Box<dyn Expression>>,
}

impl ExpressionStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> Self {
        ExpressionStatement { token, value }
    }

    pub fn value(&self) -> &Option<Box<dyn Expression>> {
        &self.value
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }

    fn get_string(&self) -> Result<String, Box<dyn Error>> {
        let mut out = String::new();

        if let Some(value) = self.value() {
            write!(out, "{}", value.get_string()?,)?;
            Ok(out)
        } else {
            let error = Box::new(ASTError::new(
                ASTErrorType::StatementError(StatementErrorType::ExpressionError),
                "Error while parsing expression statement: None valie".to_string(),
            ));
            Err(error)
        }
    }
}
