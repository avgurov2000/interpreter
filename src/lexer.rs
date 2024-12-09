use super::tokens::*;
use std::collections::HashMap;

pub struct Lexer<'a>{
    input: &'a [u8],
    position: usize, 
    read_position: usize,
    ch: Option<u8>,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let keywords = Self::init_keywords();
        let mut lexer = Lexer{
            input: input.as_bytes(),
            position: 0,
            read_position: 0,
            ch: None,
            keywords: keywords,
        };
        lexer.read_char();
        lexer
    }
    fn init_keywords() -> HashMap<String, TokenType>{
        let mut keywords = HashMap::new();
        keywords.insert("let".to_string(), TokenType::SpecialWord(SpecialWordType::Let));
        keywords.insert("fn".to_string(), TokenType::SpecialWord(SpecialWordType::Function));
        keywords.insert("True".to_string(), TokenType::SpecialWord(SpecialWordType::True));
        keywords.insert("False".to_string(), TokenType::SpecialWord(SpecialWordType::False));
        keywords.insert("return".to_string(), TokenType::SpecialWord(SpecialWordType::Return));
        keywords.insert("if".to_string(), TokenType::ControlFlow(ControlFlowType::If));
        keywords.insert("else".to_string(), TokenType::ControlFlow(ControlFlowType::Else));
        keywords.insert("switch".to_string(), TokenType::ControlFlow(ControlFlowType::Switch));
        keywords
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position +=1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        if self.ch.is_none() {
            return Token::new(TokenType::EOF, None )
        };
        let token = match self.ch.unwrap() {
            b'*' => Token::new(
                TokenType::Punctuation(PunctuationType::Asterisk), 
                None
            ),
            b'=' => Token::new(
                TokenType::Punctuation(PunctuationType::Assign), 
                None
            ),
            b'<' => Token::new(
                TokenType::Punctuation(PunctuationType::Less), 
                None
            ),
            b'>' => Token::new(
                TokenType::Punctuation(PunctuationType::Greater), 
                None
            ),
            b':' => Token::new(
                TokenType::Punctuation(PunctuationType::Colon), 
                None,
            ),
            b';' => Token::new(
                TokenType::Punctuation(PunctuationType::Semicolon), 
                None,
            ),
            b'(' => Token::new(
                TokenType::Punctuation(PunctuationType::LParen), 
                None
            ),
            b')' => Token::new(
                TokenType::Punctuation(PunctuationType::RParen), 
                None,
            ),
            b',' => Token::new(
                TokenType::Punctuation(PunctuationType::Comma), 
                None,
            ),
            b'+' => Token::new(
                TokenType::Punctuation(PunctuationType::Plus), 
                None,
            ),
            b'-' => Token::new(
                TokenType::Punctuation(PunctuationType::Minus), 
                None,
            ),
            b'{' => Token::new(
                TokenType::Punctuation(PunctuationType::LBrace), 
                None
            ),
            b'}' => Token::new(
                TokenType::Punctuation(PunctuationType::RBrace), 
                None
            ),
            b'&' => Token::new(
                TokenType::Punctuation(PunctuationType::Ampersand), 
                None
            ),
            _ =>  {
                if Self::is_letter(self.ch) {
                    let token_literal = self.read_identifier();
                    let token_type = self.lookup_ident(&token_literal);
                    return Token::new(token_type, Some(token_literal) )
                } else if Self::is_digit(self.ch) {
                    return Token::new(
                        TokenType::Data(DataType::Int), 
                        Some(self.read_number()),
                    )
                }else {
                    Token::new(TokenType::Illegal, None )
                }   
            },
        };
        self.read_char();
        token
    }
}



impl<'a> Lexer<'a> {
    fn is_letter(ch: Option<u8>) -> bool {
        if let Some(i) = ch {
            ('a' as u8) <= i && i <= ('z' as u8) || ('A' as u8) <= i && i <= ('Z' as u8)
        } else {false}
    }
    fn is_underscore(ch: Option<u8>) -> bool {
        if let Some(i) = ch {
            i == ('_' as u8)
        } else {false}
    }

    fn is_digit(ch: Option<u8>) -> bool {
        if let Some(i) = ch {
            ('0' as u8) <= i && i <= ('9' as u8)
        } else {false}
    }

    fn is_identifier(ch: Option<u8>, relativa_position: i32) -> bool {
        if Self::is_letter(ch) {
            true
        } else if (Self::is_digit(ch) || Self::is_underscore(ch)) && relativa_position > 0 {
            true
        } else {
            false
        }
    }

    fn skip_white_space(&mut self) {
        while self.ch == Some(b' ') || self.ch == Some(b'\t') || self.ch == Some(b'\n') || self.ch == Some(b'\r'){
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        let mut relative_position = 0;
        while Self::is_identifier(self.ch, relative_position) {
            self.read_char();
            relative_position += 1;
        }
        let identifier = self.input.get(position..self.position);
        String::from_utf8(identifier.unwrap().to_vec()).ok().unwrap()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        let identifier = self.input.get(position..self.position);
        String::from_utf8(identifier.unwrap().to_vec()).ok().unwrap()
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
    fn read_punctuation(&mut self) -> Option<Token> {
        let token = match self.ch.unwrap() {
            b'*' => Token::new(
                TokenType::Punctuation(PunctuationType::Asterisk), 
                None
            ),
            b'=' => Token::new(
                TokenType::Punctuation(PunctuationType::Assign), 
                None
            ),
            b'<' => Token::new(
                TokenType::Punctuation(PunctuationType::Less), 
                None
            ),
            b'>' => Token::new(
                TokenType::Punctuation(PunctuationType::Greater), 
                None
            ),
            b':' => Token::new(
                TokenType::Punctuation(PunctuationType::Colon), 
                None,
            ),
            b';' => Token::new(
                TokenType::Punctuation(PunctuationType::Semicolon), 
                None,
            ),
            b'(' => Token::new(
                TokenType::Punctuation(PunctuationType::LParen), 
                None
            ),
            b')' => Token::new(
                TokenType::Punctuation(PunctuationType::RParen), 
                None,
            ),
            b',' => Token::new(
                TokenType::Punctuation(PunctuationType::Comma), 
                None,
            ),
            b'+' => Token::new(
                TokenType::Punctuation(PunctuationType::Plus), 
                None,
            ),
            b'-' => Token::new(
                TokenType::Punctuation(PunctuationType::Minus), 
                None,
            ),
            b'{' => Token::new(
                TokenType::Punctuation(PunctuationType::LBrace), 
                None
            ),
            b'}' => Token::new(
                TokenType::Punctuation(PunctuationType::RBrace), 
                None
            ),
            b'&' => Token::new(
                TokenType::Punctuation(PunctuationType::Ampersand), 
                None
            ),
            _ => return None,
        };
    Some(token)
    }
}