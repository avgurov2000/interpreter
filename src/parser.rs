use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::{
    ast::{Identifier, LetStatement, Program, Statement},
    tokens::{PunctuationType, SpecialWordType, TokenType},
};

use super::{lexer::Lexer, tokens::Token};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing Error: {}", self.message)
    }
}

impl Error for ParsingError {}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let parser = Parser {
            lexer,
            current_token,
            peek_token,
        };
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Program, Box<dyn Error>> {
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while self.current_token.get_type() != TokenType::EOF {
            let stmt = self.parse_statement();
            if let Some(i) = stmt {
                statements.push(i);
            }
            self.next_token();
        }
        Ok(Program::new(statements))
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.get_type() {
            TokenType::SpecialWord(SpecialWordType::Let) => self
                .parse_let_statement()
                .map(|stmt| stmt as Box<dyn Statement>),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let stmt = LetStatement::new(
            self.current_token.clone(),
            Identifier::new(self.current_token.clone()),
            None,
        );

        if !self.expect_peek(TokenType::Punctuation(PunctuationType::Assign)) {
            return None;
        }

        loop {
            if !self.current_token_is(TokenType::Punctuation(PunctuationType::Semicolon)) {
                self.next_token();
            } else {
                break;
            }
        }
        return Some(Box::new(stmt));
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.get_type() == token_type
    }
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.get_type() == token_type
    }
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
