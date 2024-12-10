use std::any::Any;

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
}
