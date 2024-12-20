use std::any::Any;
use std::error::Error;
use std::fmt::Write;

use crate::ast::error::{ASTError, ASTErrorType, StatementErrorType};

use super::super::super::tokens::Token;
use super::super::{Expression, Node, Statement};

pub struct ReturnStatement {
    token: Token,
    value: Option<Box<dyn Expression>>,
}
impl ReturnStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> Self {
        ReturnStatement { token, value }
    }

    pub fn value(&self) -> &Option<Box<dyn Expression>> {
        &self.value
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }

    fn get_string(&self) -> Result<String, Box<dyn Error>> {
        let mut out = String::new();

        write!(
            out,
            "{}",
            self.token_literal().ok_or("Unable to read token literal")?,
        )?;

        if let Some(value) = self.value() {
            write!(out, " {}", value.get_string()?,)?;
            write!(out, ";",)?;
            Ok(out)
        } else {
            let error = Box::new(ASTError::new(
                ASTErrorType::StatementError(StatementErrorType::ReturnError),
                "Error while parsing return statement: None valie".to_string(),
            ));
            Err(error)
        }
    }
}
