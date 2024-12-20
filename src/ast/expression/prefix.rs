use std::any::Any;
use std::error::Error;
use std::fmt::Write;

use super::super::{Expression, Node};
use crate::{
    ast::error::{ASTError, ASTErrorType, ExpressionErrorType},
    tokens::{PunctuationType, Token, TokenType},
};

pub struct PrefixExpression {
    token: Token,
    value: Option<Box<dyn Expression>>,
}

impl PrefixExpression {
    pub fn value(&self) -> &Option<Box<dyn Expression>> {
        &self.value
    }

    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> Self {
        PrefixExpression { token, value }
    }

    fn get_operator_string(&self) -> Result<String, Box<dyn Error>> {
        match self.token.get_type() {
            TokenType::Punctuation(PunctuationType::Minus) => Ok("-".to_string()),
            TokenType::Punctuation(PunctuationType::Exclamation) => Ok("!".to_string()),
            _ => {
                let error = Box::new(ASTError::new(
                    ASTErrorType::ExpressionError(ExpressionErrorType::PrefixError),
                    "Error while parsing prefix expression: incorrect operator".to_string(),
                ));
                Err(error)
            }
        }
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Node for PrefixExpression {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }

    fn get_string(&self) -> Result<String, Box<dyn Error>> {
        let mut out = String::new();
        write!(out, "({}", self.get_operator_string()?)?;

        if let Some(value) = self.value() {
            write!(out, "{})", value.get_string()?,)?;
            Ok(out)
        } else {
            let error = Box::new(ASTError::new(
                ASTErrorType::ExpressionError(ExpressionErrorType::PrefixError),
                "Error while parsing return statement: None valie".to_string(),
            ));
            Err(error)
        }
    }
}
