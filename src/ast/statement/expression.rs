use std::any::Any;

use super::super::super::tokens::Token;
use super::super::{Expression, Node, Statement};

pub struct ExpressionStatement {
    token: Token,
    expression: Option<Box<dyn Expression>>,
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
}
