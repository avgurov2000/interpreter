#[cfg(test)]
mod test {

    use interpreter::ast::{LetStatement, Node, Statement};
    use interpreter::lexer::Lexer;
    use interpreter::parsing::Parser;

    type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
    #[test]
    fn test_correct_let_statements() -> TestResult<()> {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();


        assert!(
            !check_parsing_error(&parser),
            "Parsing must not contain error",
        );
        assert!(program.is_ok(), "Program parsing returned error {:?}", program.err().unwrap());
        assert!(
            program.as_ref().unwrap().len() == 3, 
            "Program statements must contain 3 elements, got {}",
            program.as_ref().unwrap().len(),
        );

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

    #[test]
    fn test_incorrect_let_statement() {
        let input = "
            let x  5;
            let = 10;
            let 838383;
        ";

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();


        assert!(
            check_parsing_error(&parser),
            "Parsing must contains error",
        );

        assert!(
            program.as_ref().unwrap().len() == 0, 
            "Program statements must contain 0 elements, got {}",
            program.as_ref().unwrap().len(),
        );
    }


    fn check_parsing_error(parser: &Parser) -> bool {
        if parser.get_errors().len() == 0 {
            false
        } else {
            for err in parser.get_errors() {
                eprintln!("Found parsing error: \n{}", err);
            }
            true
        }

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
