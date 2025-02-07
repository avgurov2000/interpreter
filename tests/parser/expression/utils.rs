#[cfg(test)]
pub mod utils_fn {

    use crate::parser::utils::utils_fn::check_parsing_error;
    use interpreter::lexer::Lexer;
    use interpreter::{ast::PrefixExpression, parsing::Parser};

    pub struct TestPrefixExpression<'a> {
        input: &'a str,
        operator: &'a str,
        integer_value: i32,
    }

    impl TestPrefixExpression<'_> {
        fn parse_correct(&self) {
            let mut lexer = Lexer::new(self.input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse();

            assert!(
                !check_parsing_error(&parser),
                "Expression parsing must not result in error",
            );
            assert!(
                program.is_ok(),
                "Expression parsing returned error {:?}",
                program.err().unwrap()
            );

            let program = program.unwrap();
            assert!(
                program.len() == 1,
                "Program statements must contain 1 elements, got {}",
                program.len(),
            );

            let expression = program
                .get_item(0)
                .as_any()
                .downcast_ref::<PrefixExpression>();
            assert!(
                expression.is_some(),
                "Expression statement downcasting must be successful, got failure",
            );
        }
    }
}
