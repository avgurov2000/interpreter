use std::any::Any;
use std::error::Error;
use std::fmt::Write;

use crate::ast::error::{ASTError, ASTErrorType, StatementErrorType};

use super::super::super::tokens::Token;
use super::super::{Expression, Identifier, Node, Statement};

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Option<Box<dyn Expression>>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Option<Box<dyn Expression>>) -> Self {
        LetStatement { token, name, value }
    }
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn value(&self) -> &Option<Box<dyn Expression>> {
        &self.value
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Node for LetStatement {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }

    fn get_string(&self) -> Result<String, Box<dyn Error>> {
        let mut out = String::new();

        // TODO Add literal reading Error
        write!(
            out,
            "{} {}",
            self.token_literal().ok_or("Unable to read token literal")?,
            self.name()
                .token_literal()
                .ok_or("Unable to read token literal")?,
        )?;

        if let Some(value) = self.value() {
            write!(out, " = {}", value.get_string()?,)?;
            write!(out, ";",)?;
            Ok(out)
        } else {
            let error = Box::new(ASTError::new(
                ASTErrorType::StatementError(StatementErrorType::LetError),
                "Error while parsing return statement: None valie".to_string(),
            ));
            Err(error)
        }
    }
}
