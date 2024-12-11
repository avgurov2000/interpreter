use std::any::Any;
use std::error::Error;

use super::super::super::tokens::Token;
use super::super::{Expression, Node};

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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        self.token.get_ch()
    }

    fn get_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.token_literal().unwrap())
    }
}
