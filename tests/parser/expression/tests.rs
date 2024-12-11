#[cfg(test)]
pub mod test_identifier {
    use crate::parser::utils::utils_fn::check_parsing_error;
    use interpreter::lexer::Lexer;
    use interpreter::parsing::Parser;

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
        assert!(
            program.as_ref().unwrap().len() == 1,
            "Program statements must contain 1 elements, got {}",
            program.as_ref().unwrap().len(),
        );
    }
}
