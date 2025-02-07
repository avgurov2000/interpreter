#[cfg(test)]
pub mod test_identifier {
    use crate::parser::utils::utils_fn::check_parsing_error;
    use interpreter::lexer::Lexer;
    use interpreter::{
        ast::{ExpressionStatement, Identifier, Node},
        parsing::Parser,
    };

    #[test]
    fn test_correct_identifier() {
        let input = "foobar;";

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

        let program = program.unwrap();
        assert!(
            program.len() == 1,
            "Program statements must contain 1 elements, got {}",
            program.len(),
        );

        let expression_statement = program
            .get_item(0)
            .as_any()
            .downcast_ref::<ExpressionStatement>();
        assert!(
            expression_statement.is_some(),
            "Expression statement downcasting must be successful, got failure",
        );

        let expression_statement = expression_statement.unwrap();

        let token_literal = expression_statement.token_literal();
        assert!(
            token_literal.is_some(),
            "Expression statement token literal must be Som(_), got None",
        );
        assert!(
            token_literal.clone().unwrap() == "foobar",
            "Expression statement token literal value must be {}, got {}",
            "foobar",
            token_literal.clone().unwrap(),
        );

        let identifier = expression_statement.value();
        assert!(
            identifier.is_some(),
            "Identifier of the expression statement must be Som(_), got None",
        );
        let identifier = identifier
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Identifier>();
        assert!(
            identifier.is_some(),
            "Identifier downcasting must be successful, got failure",
        );
        let token_literal = identifier.unwrap().token_literal();
        assert!(
            token_literal.is_some(),
            "Identifier token literal must be Som(_), got None",
        );
        assert!(
            token_literal.clone().unwrap() == "foobar",
            "Identifier token literal value must be {}, got {}",
            "foobar",
            token_literal.clone().unwrap(),
        );
    }
}

pub mod test_integet_literal {
    use crate::parser::utils::utils_fn::check_parsing_error;
    use interpreter::ast::IntegerLiteral;
    use interpreter::lexer::Lexer;
    use interpreter::{
        ast::{ExpressionStatement, Node},
        parsing::Parser,
    };

    #[test]
    fn test_correct_integet_literal() {
        let input = "27;";

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

        let program = program.unwrap();
        assert!(
            program.len() == 1,
            "Program statements must contain 1 elements, got {}",
            program.len(),
        );

        let expression_statement = program
            .get_item(0)
            .as_any()
            .downcast_ref::<ExpressionStatement>();
        assert!(
            expression_statement.is_some(),
            "Expression statement downcasting must be successful, got failure",
        );

        let expression_statement = expression_statement.unwrap();

        let token_literal = expression_statement.token_literal();
        assert!(
            token_literal.is_some(),
            "Expression statement token literal must be Som(_), got None",
        );
        assert!(
            token_literal.clone().unwrap() == "27",
            "Expression statement token literal value must be {}, got {}",
            "27",
            token_literal.clone().unwrap(),
        );

        let integer_literal = expression_statement.value();
        assert!(
            integer_literal.is_some(),
            "Integer literal of the expression statement must be Som(_), got None",
        );

        let integer_literal = integer_literal
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<IntegerLiteral>();
        assert!(
            integer_literal.is_some(),
            "Integer literal downcasting must be successful, got failure",
        );

        let integer_value = integer_literal.unwrap().value();
        eprintln!("{:?}", integer_value);
        assert!(
            integer_value == 27,
            "Integer literal value must be {}, got {}",
            27,
            integer_value,
        );
    }
}

pub mod test_prefix_expression {}
