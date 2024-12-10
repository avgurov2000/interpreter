use super::Statement;

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
