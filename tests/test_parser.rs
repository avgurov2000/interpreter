#[cfg(test)]
mod test {

    use interpreter::ast::{LetStatement, Node, Statement};
    use interpreter::lexer::Lexer;
    use interpreter::parser::Parser;

    type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
    #[test]
    fn test_let_statements() -> TestResult<()> {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        if let Err(msg) = program {
            panic!("Program parsing returned error {:?}", msg);
        } else if program.as_ref().unwrap().len() != 3 {
            panic!(
                "program statements must contain 3 elements, got {}",
                program.as_ref().unwrap().len()
            );
        }

        let program = program.unwrap();

        let expected_literals = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];

        for (idx, expected_literal) in expected_literals.into_iter().enumerate() {
            assert!(test_let_statement(
                program.get_item(idx).as_ref(),
                expected_literal,
            ))
        }

        Ok(())
    }

    fn is_let_statement(statement: &dyn Statement) -> bool {
        statement.as_any().downcast_ref::<LetStatement>().is_some()
    }
    fn test_let_statement(statement: &dyn Statement, name: String) -> bool {
        if let Some(token_literal) = statement.token_literal() {
            assert!(
                token_literal == name,
                "Token literal '{}' is not equal to ground truth '{}'",
                token_literal,
                name,
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
