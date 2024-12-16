use std::any::Any;
use std::error::Error;

use super::super::super::tokens::Token;
use super::super::{Expression, Node};

pub struct IntegerLiteral {
    token: Token,
    value: i32,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i32) -> Self {
        IntegerLiteral { token, value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }

    fn get_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.token_literal().unwrap())
    }
}
