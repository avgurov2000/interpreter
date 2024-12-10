#[cfg(test)]
pub mod test_let {
    use super::super::utils::utils_fn::*;
    use interpreter::{lexer::Lexer, parsing::Parser};

    #[test]
    fn test_correct_let_statements() -> TestResult<()> {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let mut lexer = Lexer::new(input);
        let mut parser = interpreter::parsing::Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(
            !check_parsing_error(&parser),
            "Parsing must not contain error",
        );
        assert!(
            program.is_ok(),
            "Program parsing returned error {:?}",
            program.err().unwrap()
        );
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

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(check_parsing_error(&parser), "Parsing must contains error",);
        assert!(parser.get_errors().len() == 3);

        assert!(
            program.as_ref().unwrap().len() == 0,
            "Program statements must contain 0 elements, got {}",
            program.as_ref().unwrap().len(),
        );
    }
}

pub mod test_return {

    use super::super::utils::utils_fn::*;
    use interpreter::{lexer::Lexer, parsing::Parser};

    #[test]
    fn test_correct_return_statement() -> TestResult<()> {
        let input = "
        return 5;
        return 10;
        return 993322;
        ";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse();

        assert!(
            !check_parsing_error(&parser),
            "Parsing must not contain error",
        );

        assert!(
            program.is_ok(),
            "Program parsing returned error {:?}",
            program.err().unwrap()
        );
        assert!(
            program.as_ref().unwrap().len() == 3,
            "Program statements must contain 3 elements, got {}",
            program.as_ref().unwrap().len(),
        );

        let program = program.unwrap();
        for idx in 0..program.len() {
            assert!(
                is_return_statement(program.get_item(idx).as_ref()),
                "Statement must be 'ReturnStatement' type",
            )
        }
        Ok(())
    }
}
