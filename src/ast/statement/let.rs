use std::any::Any;

use super::super::super::tokens::Token;
use super::super::{Expression, Node, Statement};

pub struct Identifier {
    token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token }
    }
}
impl Expression for Identifier {
    fn expression_node(&self) {}
}
impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }
}

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
