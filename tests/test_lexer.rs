#[cfg(test)]
mod test {

    use interpreter::lexer::Lexer;
    use interpreter::tokens::{
        ControlFlowType, DataType, PunctuationType, SpecialWordType, Token, TokenType,
    };
    type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

    fn create_token_full(
        token_type: TokenType,
        ch: Option<String>,
        row_position: usize,
        characher_position: usize,
    ) -> Token {
        Token::new(token_type, ch, row_position, characher_position)
    }

    fn create_token_short(token_type: TokenType, ch: Option<String>) -> Token {
        Token::new(token_type, ch, 0, 0)
    }

    #[test]
    fn test_lexer_illegal() -> TestResult<()> {
        let input = "=+(){},;Привет;";
        let expected_tokens = vec![
            create_token_full(TokenType::Punctuation(PunctuationType::Assign), None, 0, 0),
            create_token_full(TokenType::Punctuation(PunctuationType::Plus), None, 0, 1),
            create_token_full(TokenType::Punctuation(PunctuationType::LParen), None, 0, 2),
            create_token_full(TokenType::Punctuation(PunctuationType::RParen), None, 0, 3),
            create_token_full(TokenType::Punctuation(PunctuationType::LBrace), None, 0, 4),
            create_token_full(TokenType::Punctuation(PunctuationType::RBrace), None, 0, 5),
            create_token_full(TokenType::Punctuation(PunctuationType::Comma), None, 0, 6),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                0,
                7,
            ),
            create_token_full(TokenType::Illegal, Some("208".to_string()), 0, 8),
            create_token_full(TokenType::Illegal, Some("159".to_string()), 0, 9),
            create_token_full(TokenType::Illegal, Some("209".to_string()), 0, 10),
            create_token_full(TokenType::Illegal, Some("128".to_string()), 0, 11),
            create_token_full(TokenType::Illegal, Some("208".to_string()), 0, 12),
            create_token_full(TokenType::Illegal, Some("184".to_string()), 0, 13),
            create_token_full(TokenType::Illegal, Some("208".to_string()), 0, 14),
            create_token_full(TokenType::Illegal, Some("178".to_string()), 0, 15),
            create_token_full(TokenType::Illegal, Some("208".to_string()), 0, 16),
            create_token_full(TokenType::Illegal, Some("181".to_string()), 0, 17),
            create_token_full(TokenType::Illegal, Some("209".to_string()), 0, 18),
            create_token_full(TokenType::Illegal, Some("130".to_string()), 0, 19),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                0,
                20,
            ),
            create_token_full(TokenType::EOF, None, 0, 21),
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
            assert_eq!(token.get_row_position(), expected_token.get_row_position());
            assert_eq!(
                token.get_character_position(),
                expected_token.get_character_position()
            );
        }
        Ok(())
    }

    #[test]
    fn test_lexer_premitives_with_positions() -> TestResult<()> {
        let input = "=+()=={},;!///!=";

        let expected_tokens = vec![
            create_token_full(TokenType::Punctuation(PunctuationType::Assign), None, 0, 0),
            create_token_full(TokenType::Punctuation(PunctuationType::Plus), None, 0, 1),
            create_token_full(TokenType::Punctuation(PunctuationType::LParen), None, 0, 2),
            create_token_full(TokenType::Punctuation(PunctuationType::RParen), None, 0, 3),
            create_token_full(TokenType::Punctuation(PunctuationType::Equal), None, 0, 4),
            create_token_full(TokenType::Punctuation(PunctuationType::LBrace), None, 0, 6),
            create_token_full(TokenType::Punctuation(PunctuationType::RBrace), None, 0, 7),
            create_token_full(TokenType::Punctuation(PunctuationType::Comma), None, 0, 8),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                0,
                9,
            ),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Exclamation),
                None,
                0,
                10,
            ),
            create_token_full(
                TokenType::Punctuation(PunctuationType::DoubleSlash),
                None,
                0,
                11,
            ),
            create_token_full(TokenType::Punctuation(PunctuationType::Slash), None, 0, 13),
            create_token_full(
                TokenType::Punctuation(PunctuationType::NotEqual),
                None,
                0,
                14,
            ),
            create_token_full(TokenType::EOF, None, 0, 16),
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
            assert_eq!(token.get_row_position(), expected_token.get_row_position());
            assert_eq!(
                token.get_character_position(),
                expected_token.get_character_position()
            );
        }

        Ok(())
    }

    #[test]
    fn test_lexer_key_words() -> TestResult<()> {
        let input = "     let five = (2+3);
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result_5_plus_10 = add(five, ten);
        result_5_plus_10 == 15";

        let expected_tokens = vec![
            create_token_short(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
            ),
            create_token_short(TokenType::Ident, Some("five".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Assign), None),
            create_token_short(TokenType::Punctuation(PunctuationType::LParen), None),
            create_token_short(TokenType::Data(DataType::Int), Some("2".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Plus), None),
            create_token_short(TokenType::Data(DataType::Int), Some("3".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::RParen), None),
            create_token_short(TokenType::Punctuation(PunctuationType::Semicolon), None),
            create_token_short(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
            ),
            create_token_short(TokenType::Ident, Some("ten".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Assign), None),
            create_token_short(TokenType::Data(DataType::Int), Some("10".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Semicolon), None),
            create_token_short(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
            ),
            create_token_short(TokenType::Ident, Some("add".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Assign), None),
            create_token_short(
                TokenType::SpecialWord(SpecialWordType::Function),
                Some("fn".to_string()),
            ),
            create_token_short(TokenType::Punctuation(PunctuationType::LParen), None),
            create_token_short(TokenType::Ident, Some("x".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Comma), None),
            create_token_short(TokenType::Ident, Some("y".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::RParen), None),
            create_token_short(TokenType::Punctuation(PunctuationType::LBrace), None),
            create_token_short(TokenType::Ident, Some("x".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Plus), None),
            create_token_short(TokenType::Ident, Some("y".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Semicolon), None),
            create_token_short(TokenType::Punctuation(PunctuationType::RBrace), None),
            create_token_short(TokenType::Punctuation(PunctuationType::Semicolon), None),
            create_token_short(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
            ),
            create_token_short(TokenType::Ident, Some("result_5_plus_10".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Assign), None),
            create_token_short(TokenType::Ident, Some("add".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::LParen), None),
            create_token_short(TokenType::Ident, Some("five".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Comma), None),
            create_token_short(TokenType::Ident, Some("ten".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::RParen), None),
            create_token_short(TokenType::Punctuation(PunctuationType::Semicolon), None),
            create_token_short(TokenType::Ident, Some("result_5_plus_10".to_string())),
            create_token_short(TokenType::Punctuation(PunctuationType::Equal), None),
            create_token_short(TokenType::Data(DataType::Int), Some("15".to_string())),
            create_token_short(TokenType::EOF, None),
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
        }

        Ok(())
    }

    #[test]
    fn test_lexer_controlflow_with_positions() -> TestResult<()> {
        let input = "let x = 12;\n
let y = 22;\n
let is_greater = fn(a: int, b: int) {\n
    if a > b {\n
        return True;\n
    } else {\n
        return False;\n
    }\n
}\n
x != y\n
";

        let expected_tokens = vec![
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
                0,
                0,
            ),
            create_token_full(TokenType::Ident, Some("x".to_string()), 0, 4),
            create_token_full(TokenType::Punctuation(PunctuationType::Assign), None, 0, 6),
            create_token_full(TokenType::Data(DataType::Int), Some("12".to_string()), 0, 8),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                0,
                10,
            ),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
                1,
                0,
            ),
            create_token_full(TokenType::Ident, Some("y".to_string()), 1, 4),
            create_token_full(TokenType::Punctuation(PunctuationType::Assign), None, 1, 6),
            create_token_full(TokenType::Data(DataType::Int), Some("22".to_string()), 1, 8),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                1,
                10,
            ),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::Let),
                Some("let".to_string()),
                2,
                0,
            ),
            create_token_full(TokenType::Ident, Some("is_greater".to_string()), 2, 4),
            create_token_full(TokenType::Punctuation(PunctuationType::Assign), None, 2, 15),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::Function),
                Some("fn".to_string()),
                2,
                17,
            ),
            create_token_full(TokenType::Punctuation(PunctuationType::LParen), None, 2, 19),
            create_token_full(TokenType::Ident, Some("a".to_string()), 2, 20),
            create_token_full(TokenType::Punctuation(PunctuationType::Colon), None, 2, 21),
            create_token_full(TokenType::Ident, Some("int".to_string()), 2, 23),
            create_token_full(TokenType::Punctuation(PunctuationType::Comma), None, 2, 26),
            create_token_full(TokenType::Ident, Some("b".to_string()), 2, 28),
            create_token_full(TokenType::Punctuation(PunctuationType::Colon), None, 2, 29),
            create_token_full(TokenType::Ident, Some("int".to_string()), 2, 31),
            create_token_full(TokenType::Punctuation(PunctuationType::RParen), None, 2, 34),
            create_token_full(TokenType::Punctuation(PunctuationType::LBrace), None, 2, 36),
            create_token_full(
                TokenType::ControlFlow(ControlFlowType::If),
                Some("if".to_string()),
                3,
                4,
            ),
            create_token_full(TokenType::Ident, Some("a".to_string()), 3, 7),
            create_token_full(TokenType::Punctuation(PunctuationType::Greater), None, 3, 9),
            create_token_full(TokenType::Ident, Some("b".to_string()), 3, 11),
            create_token_full(TokenType::Punctuation(PunctuationType::LBrace), None, 3, 13),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::Return),
                Some("return".to_string()),
                4,
                8,
            ),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::True),
                Some("True".to_string()),
                4,
                15,
            ),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                4,
                19,
            ),
            create_token_full(TokenType::Punctuation(PunctuationType::RBrace), None, 5, 4),
            create_token_full(
                TokenType::ControlFlow(ControlFlowType::Else),
                Some("else".to_string()),
                5,
                6,
            ),
            create_token_full(TokenType::Punctuation(PunctuationType::LBrace), None, 5, 11),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::Return),
                Some("return".to_string()),
                6,
                8,
            ),
            create_token_full(
                TokenType::SpecialWord(SpecialWordType::False),
                Some("False".to_string()),
                6,
                15,
            ),
            create_token_full(
                TokenType::Punctuation(PunctuationType::Semicolon),
                None,
                6,
                20,
            ),
            create_token_full(TokenType::Punctuation(PunctuationType::RBrace), None, 7, 4),
            create_token_full(TokenType::Punctuation(PunctuationType::RBrace), None, 8, 0),
            create_token_full(TokenType::Ident, Some("x".to_string()), 9, 0),
            create_token_full(
                TokenType::Punctuation(PunctuationType::NotEqual),
                None,
                9,
                2,
            ),
            create_token_full(TokenType::Ident, Some("y".to_string()), 9, 5),
            create_token_full(TokenType::EOF, None, 10, 0),
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.get_type(), expected_token.get_type());
            assert_eq!(token.get_ch(), expected_token.get_ch());
            assert_eq!(token.get_row_position(), expected_token.get_row_position());
            assert_eq!(
                token.get_character_position(),
                expected_token.get_character_position()
            );
        }

        Ok(())
    }
}
