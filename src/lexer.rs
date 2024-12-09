use super::tokens::*;
use std::collections::HashMap;

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<u8>,
    keywords: HashMap<String, TokenType>,
    file_position: (usize, usize),
    read_file_position: (usize, usize),
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let keywords = Self::init_keywords();
        let mut lexer = Lexer {
            input: input.as_bytes(),
            position: 0,
            read_position: 0,
            ch: None,
            keywords: keywords,
            file_position: (0, 0),
            read_file_position: (0, 0),
        };
        lexer.read_char();
        lexer
    }

    pub fn get_row_position(&self) -> usize {
        self.file_position.0
    }

    pub fn get_character_position(&self) -> usize {
        self.file_position.1
    }

    pub fn set_file_position(&mut self, row_position: usize, character_position: usize) {
        self.file_position = (row_position, character_position);
        self.read_file_position = (row_position, character_position);
    }

    fn init_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert(
            "let".to_string(),
            TokenType::SpecialWord(SpecialWordType::Let),
        );
        keywords.insert(
            "fn".to_string(),
            TokenType::SpecialWord(SpecialWordType::Function),
        );
        keywords.insert(
            "True".to_string(),
            TokenType::SpecialWord(SpecialWordType::True),
        );
        keywords.insert(
            "False".to_string(),
            TokenType::SpecialWord(SpecialWordType::False),
        );
        keywords.insert(
            "return".to_string(),
            TokenType::SpecialWord(SpecialWordType::Return),
        );
        keywords.insert(
            "if".to_string(),
            TokenType::ControlFlow(ControlFlowType::If),
        );
        keywords.insert(
            "else".to_string(),
            TokenType::ControlFlow(ControlFlowType::Else),
        );
        keywords.insert(
            "switch".to_string(),
            TokenType::ControlFlow(ControlFlowType::Switch),
        );
        keywords
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.position = self.read_position;
        self.file_position = self.read_file_position;

        self.read_file_position = (self.read_file_position.0, self.read_file_position.1 + 1);
        self.read_position += 1;
    }

    fn peak_char(&self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            return self.input.get(self.read_position).copied();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        if self.ch.is_none() {
            return Token::new(
                TokenType::EOF,
                None,
                self.get_row_position(),
                self.get_character_position(),
            );
        } else if let Some(i) = self.get_punctuation() {
            self.read_char();
            return i;
        } else if let Some(i) = self.get_identifier() {
            return i;
        } else if Self::is_digit(self.ch.unwrap()) {
            let (token_literal, row_position, character_position) = self.read_number();

            return Token::new(
                TokenType::Data(DataType::Int),
                Some(token_literal),
                row_position,
                character_position,
            );
        } else {
            self.read_char();
            return Token::new(
                TokenType::Illegal,
                Some(self.ch.unwrap().to_string()),
                self.get_row_position(),
                self.get_character_position(),
            );
        };
    }
}

impl<'a> Lexer<'a> {
    fn is_letter(ch: u8) -> bool {
        ('a' as u8) <= ch && ch <= ('z' as u8) || ('A' as u8) <= ch && ch <= ('Z' as u8)
    }
    fn is_underscore(ch: u8) -> bool {
        ch == ('_' as u8)
    }

    fn is_digit(ch: u8) -> bool {
        ('0' as u8) <= ch && ch <= ('9' as u8)
    }

    fn is_identifier(ch: u8, relativa_position: i32) -> bool {
        if Self::is_letter(ch) {
            true
        } else if (Self::is_digit(ch) || Self::is_underscore(ch)) && relativa_position > 0 {
            true
        } else {
            false
        }
    }

    fn skip_white_space(&mut self) {
        while self.ch == Some(b' ')
            || self.ch == Some(b'\t')
            || self.ch == Some(b'\n')
            || self.ch == Some(b'\r')
        {
            self.read_char();
            if self.ch == Some(b'\r') || self.ch == Some(b'\n') {
                self.set_file_position(self.get_row_position() + 1, 0)
            }
        }
    }

    fn read_identifier(&mut self) -> (String, usize, usize) {
        let row_position = self.get_row_position();
        let character_position = self.get_character_position();
        let position = self.position;
        let mut relative_position = 0;
        while self.ch.is_some() && Self::is_identifier(self.ch.unwrap(), relative_position) {
            self.read_char();
            relative_position += 1;
        }
        let identifier = self.input.get(position..self.position);
        (
            String::from_utf8(identifier.unwrap().to_vec())
                .ok()
                .unwrap(),
            row_position,
            character_position,
        )
    }

    fn read_number(&mut self) -> (String, usize, usize) {
        let row_position = self.get_row_position();
        let character_position = self.get_character_position();
        let position = self.position;
        while self.ch.is_some() && Self::is_digit(self.ch.unwrap()) {
            self.read_char();
        }
        let identifier = self.input.get(position..self.position);
        (
            String::from_utf8(identifier.unwrap().to_vec())
                .ok()
                .unwrap(),
            row_position,
            character_position,
        )
    }

    fn lookup_ident(&self, ident: &String) -> TokenType {
        if let Some(t_type) = self.keywords.get(ident) {
            t_type.clone()
        } else {
            TokenType::Ident
        }
    }
}

impl<'a> Lexer<'a> {
    fn get_identifier(&mut self) -> Option<Token> {
        if !Self::is_letter(self.ch.unwrap()) {
            return None;
        }
        let (token_literal, row_position, character_position) = self.read_identifier();
        let token_type = self.lookup_ident(&token_literal);
        return Some(Token::new(
            token_type,
            Some(token_literal),
            row_position,
            character_position,
        ));
    }

    fn get_punctuation(&mut self) -> Option<Token> {
        let row_position = self.get_row_position();
        let character_position = self.get_character_position();
        let token_type = match self.ch.unwrap() {
            b'&' => TokenType::Punctuation(PunctuationType::Ampersand),
            b'=' => {
                let next_symbol = self.peak_char();
                if next_symbol.is_some() && next_symbol.unwrap() == b'=' {
                    self.read_char();
                    TokenType::Punctuation(PunctuationType::Equal)
                } else {
                    TokenType::Punctuation(PunctuationType::Assign)
                }
            }
            b'*' => TokenType::Punctuation(PunctuationType::Asterisk),
            b'\\' => TokenType::Punctuation(PunctuationType::BackSlash),
            b':' => TokenType::Punctuation(PunctuationType::Colon),
            b',' => TokenType::Punctuation(PunctuationType::Comma),
            b'!' => {
                let next_symbol = self.peak_char();
                if next_symbol.is_some() && next_symbol.unwrap() == b'=' {
                    self.read_char();
                    TokenType::Punctuation(PunctuationType::NotEqual)
                } else {
                    TokenType::Punctuation(PunctuationType::Exclamation)
                }
            }
            b'>' => TokenType::Punctuation(PunctuationType::Greater),
            b'{' => TokenType::Punctuation(PunctuationType::LBrace),
            b'(' => TokenType::Punctuation(PunctuationType::LParen),
            b'<' => TokenType::Punctuation(PunctuationType::Less),
            b'-' => TokenType::Punctuation(PunctuationType::Minus),
            b'+' => TokenType::Punctuation(PunctuationType::Plus),
            b'?' => TokenType::Punctuation(PunctuationType::Question),
            b'}' => TokenType::Punctuation(PunctuationType::RBrace),
            b')' => TokenType::Punctuation(PunctuationType::RParen),
            b';' => TokenType::Punctuation(PunctuationType::Semicolon),
            b'/' => TokenType::Punctuation(PunctuationType::Slash),
            _ => return None,
        };
        Some(Token::new(
            token_type,
            None,
            row_position,
            character_position,
        ))
    }
}
