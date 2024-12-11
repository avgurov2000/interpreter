use std::any::Any;


pub trait Node {
    fn token_literal(&self) -> Option<String>;
}

pub trait Statement: Node + Any {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + Any {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}
