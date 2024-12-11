use std::any::Any;

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
}
