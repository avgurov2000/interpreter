use super::Statement;

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&self) -> Option<String> {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    pub fn get_item(&self, index: usize) -> &dyn Statement {
        self.statements[index].as_ref()
    }

    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements }
    }
}
