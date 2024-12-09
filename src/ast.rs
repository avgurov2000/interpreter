use std::any::Any;

use super::tokens::Token;

pub trait Node {
    fn token_literal(&self) -> Option<String>;
}

pub trait Statement: Node + Any {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return None;
        }
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    pub fn get_item(&self, index: usize) -> &Box<dyn Statement> {
        &self.statements[index]
    }

    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements }
    }
}

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
        return self.token.get_ch();
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
