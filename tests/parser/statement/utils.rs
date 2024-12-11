#[cfg(test)]
pub mod utils_fn {

    use interpreter::ast::{LetStatement, Node, ReturnStatement, Statement};

    pub fn is_let_statement(statement: &dyn Statement) -> bool {
        statement.as_any().downcast_ref::<LetStatement>().is_some()
    }

    pub fn is_return_statement(statement: &dyn Statement) -> bool {
        statement
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .is_some()
    }

    pub fn test_let_statement(statement: &dyn Statement, name: String) -> bool {
        if let Some(token_literal) = statement.token_literal() {
            assert!(
                token_literal == *"let",
                "Token literal '{}' is not equal to ground truth '{}'",
                token_literal,
                "let",
            );
        } else {
            return false;
        }

        assert!(
            is_let_statement(statement),
            "Statement has a type different form 'LetStatement'"
        );
        let let_statement = statement.as_any().downcast_ref::<LetStatement>().unwrap();
        if let Some(i) = let_statement.name().token_literal() {
            assert_eq!(i, name, "Value must be equal, got {} and {}", i, name);
        } else {
            return false;
        }
        true
    }
}
