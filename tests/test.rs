#[cfg(test)]
mod test {

    use interpreter::tokens::{
        DataType, PunctuationType, SpecialWordType, Token, TokenType, ControlFlowType
    };
    use interpreter::lexer::Lexer;
    type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
    
    #[test]
    fn test_lexer_premitives() -> TestResult<()>{
        let input = "=+(){},;";

        let expected_tokens = vec![
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::Punctuation(PunctuationType::Plus), None),
            Token::new(TokenType::Punctuation(PunctuationType::LParen), None),
            Token::new(TokenType::Punctuation(PunctuationType::RParen), None),
            Token::new(TokenType::Punctuation(PunctuationType::LBrace), None),
            Token::new(TokenType::Punctuation(PunctuationType::RBrace), None),
            Token::new(TokenType::Punctuation(PunctuationType::Comma), None),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),
            Token::new(TokenType::EOF, None),
        ]; 

        let mut lexer = Lexer::new(&input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
        }

        Ok(())
    }

    #[test]
    fn test_lexer_key_words() -> TestResult<()>{
        let input = "     let five = (2+3);
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result_5_plus_10 = add(five, ten);";

        let expected_tokens = vec![
            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("five".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::Punctuation(PunctuationType::LParen), None),
            Token::new(TokenType::Data(DataType::Int), Some("2".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Plus), None),
            Token::new(TokenType::Data(DataType::Int), Some("3".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::RParen), None),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),

            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("ten".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::Data(DataType::Int), Some("10".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),

            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("add".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::SpecialWord(SpecialWordType::Function), Some("fn".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::LParen), None),
            Token::new(TokenType::Ident, Some("x".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Comma),None),
            Token::new(TokenType::Ident, Some("y".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::RParen), None),
            Token::new(TokenType::Punctuation(PunctuationType::LBrace), None),
            Token::new(TokenType::Ident, Some("x".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Plus), None),
            Token::new(TokenType::Ident, Some("y".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),
            Token::new(TokenType::Punctuation(PunctuationType::RBrace), None),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),

            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("result_5_plus_10".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::Ident, Some("add".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::LParen), None),
            Token::new(TokenType::Ident, Some("five".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Comma),None),
            Token::new(TokenType::Ident, Some("ten".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::RParen), None),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),
            Token::new(TokenType::EOF, None),
        ]; 

        let mut lexer = Lexer::new(&input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
        }

        Ok(())
    }

    #[test]
    fn test_lexer_controlflow() -> TestResult<()>{
        let input = "let x = 12;
        let y = 22;
        let is_greater = fn(a: int, b: int) {
            if a > b {
                return True;
            } else {
                return False;
            }
        }
        ";

        let expected_tokens = vec![
            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("x".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::Data(DataType::Int), Some("12".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),

            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("y".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::Data(DataType::Int), Some("22".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),

            Token::new(TokenType::SpecialWord(SpecialWordType::Let), Some("let".to_string())),
            Token::new(TokenType::Ident, Some("is_greater".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Assign), None),
            Token::new(TokenType::SpecialWord(SpecialWordType::Function), Some("fn".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::LParen), None),
            Token::new(TokenType::Ident, Some("a".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Colon), None),
            Token::new(TokenType::Ident, Some("int".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Comma), None),
            Token::new(TokenType::Ident, Some("b".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Colon), None),
            Token::new(TokenType::Ident, Some("int".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::RParen), None),

            Token::new(TokenType::Punctuation(PunctuationType::LBrace), None),
            Token::new(TokenType::ControlFlow(ControlFlowType::If), Some("if".to_string())),
            Token::new(TokenType::Ident, Some("a".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Greater), None),
            Token::new(TokenType::Ident, Some("b".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::LBrace), None),
            Token::new(TokenType::SpecialWord(SpecialWordType::Return), Some("return".to_string())),
            Token::new(TokenType::SpecialWord(SpecialWordType::True), Some("True".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),
            Token::new(TokenType::Punctuation(PunctuationType::RBrace), None),
            Token::new(TokenType::ControlFlow(ControlFlowType::Else), Some("else".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::LBrace), None),
            Token::new(TokenType::SpecialWord(SpecialWordType::Return), Some("return".to_string())),
            Token::new(TokenType::SpecialWord(SpecialWordType::False), Some("False".to_string())),
            Token::new(TokenType::Punctuation(PunctuationType::Semicolon), None),
            Token::new(TokenType::Punctuation(PunctuationType::RBrace), None),
            Token::new(TokenType::Punctuation(PunctuationType::RBrace), None),
            Token::new(TokenType::EOF, None),
        ]; 

        let mut lexer = Lexer::new(&input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
        }

        Ok(())
    }
}