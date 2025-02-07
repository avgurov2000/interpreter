use log::debug;

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program,
        ReturnStatement, Statement,
    },
    tokens::DataType,
};

use super::super::{
    lexer::Lexer,
    tokens::{PunctuationType, SpecialWordType, Token, TokenType},
};
use super::error::{ParsingError, ParsingErrorType};
use super::precedence::Precedence;
use std::collections::HashMap;
use std::error::Error;

type PrefixParserFn = fn(&mut Parser<'_>) -> Option<Box<dyn Expression>>;
type InfixParserFn = fn(&mut Parser<'_>, Box<dyn Expression>) -> Box<dyn Expression>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParsingError>,
    prefix_functions: HashMap<TokenType, PrefixParserFn>,
    infix_functions: HashMap<TokenType, InfixParserFn>,
}

// Main interface
impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let prefix_functions = HashMap::new();
        let infix_functions = HashMap::new();

        let mut parser = Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
            prefix_functions,
            infix_functions,
        };

        parser.register_prefix(TokenType::Ident, |p| p.parse_identifier());
        parser.register_prefix(TokenType::Data(DataType::Int), |p| {
            p.parse_integer_literal()
        });
        parser
    }

    fn register_prefix(&mut self, toke_type: TokenType, funtion: PrefixParserFn) {
        self.prefix_functions.insert(toke_type, funtion);
    }

    fn register_infix(&mut self, toke_type: TokenType, funtion: InfixParserFn) {
        self.infix_functions.insert(toke_type, funtion);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Program, Box<dyn Error>> {
        debug!("Parsing started");
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while self.current_token.get_type() != TokenType::EOF {
            debug!(
                "Current token -- type: {:?}, literal: {:?}",
                self.current_token.get_type(),
                self.current_token.get_ch(),
            );
            if let Some(i) = self.parse_statement() {
                statements.push(i);
            }
            self.next_token();
        }
        Ok(Program::new(statements))
    }

    fn current_token_is(&self, token_type: &TokenType) -> bool {
        self.current_token.get_type() == *token_type
    }
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.get_type() == *token_type
    }
}
// Main interface

// Parsing expressoins
impl Parser<'_> {
    fn parse_expressoin(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        if let Some(prefix) = self.prefix_functions.get(&self.current_token.get_type()) {
            prefix(self)
        } else {
            self.no_prefix_error(&self.current_token.get_type());
            None
        }
    }

    pub fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let current_token = self.current_token.clone();
        let mut error_msg = "".to_string();
        if let Some(i) = current_token.get_ch() {
            let current_value = i.parse::<i32>();
            if current_value.is_ok() {
                let value = current_value.unwrap();
                return Some(Box::new(IntegerLiteral::new(current_token, value)));
            } else {
                error_msg = format!("Invalid conversion {} from string to integer.", i);
            }
        } else {
            error_msg = "Empty integer token literal".to_string();
        }
        let error = ParsingError::new(ParsingErrorType::DataError, error_msg);
        self.push_error(error);
        None
    }

    pub fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier::new(self.current_token.clone())))
    }
}
// Parsing expressoins

// Parsing statements
impl Parser<'_> {
    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.get_type() {
            TokenType::SpecialWord(SpecialWordType::Let) => {
                debug!("{}Let statement parsing", "\t");
                self.parse_let_statement()
                    .map(|stmt| stmt as Box<dyn Statement>)
            }
            TokenType::SpecialWord(SpecialWordType::Return) => {
                debug!("{}Return statement parsing", "\t");
                self.parse_return_statement()
                    .map(|stmt| stmt as Box<dyn Statement>)
            }
            _ => {
                debug!("{}Expression statement parsing", "\t");
                self.parse_expression_statement()
                    .map(|stmt| stmt as Box<dyn Statement>)
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        let current_token = self.current_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let stmt = LetStatement::new(
            current_token,
            Identifier::new(self.current_token.clone()),
            None,
        );

        if !self.expect_peek(TokenType::Punctuation(PunctuationType::Assign)) {
            return None;
        }

        loop {
            if !self.current_token_is(&TokenType::Punctuation(PunctuationType::Semicolon)) {
                self.next_token();
            } else {
                break;
            }
        }
        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<ReturnStatement>> {
        let current_token = self.current_token.clone();

        self.next_token();

        while !self.current_token_is(&TokenType::Punctuation(PunctuationType::Semicolon)) {
            self.next_token();
        }
        Some(Box::new(ReturnStatement::new(current_token, None)))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<ExpressionStatement>> {
        let current_token = self.current_token.clone();
        let current_expression = self.parse_expressoin(Precedence::Lowest);

        let expression = ExpressionStatement::new(current_token, current_expression);

        if self.peek_token_is(&TokenType::Punctuation(PunctuationType::Semicolon)) {
            self.next_token();
        }
        Some(Box::new(expression))
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&token_type);
            false
        }
    }
}
// Parsing statements

// Adding parsing error
impl Parser<'_> {
    pub fn get_errors(&self) -> &Vec<ParsingError> {
        &self.errors
    }
    pub fn push_error(&mut self, error: ParsingError) {
        self.errors.push(error);
    }

    fn no_prefix_error(&mut self, token_type: &TokenType) {
        let error = ParsingError::new(
            ParsingErrorType::PrefixError,
            format!("No prefix type parse function for {:?} found", token_type,),
        );
        self.push_error(error);
    }

    fn peek_error(&mut self, token_type: &TokenType) {
        let error = ParsingError::new(
            ParsingErrorType::PeekError,
            format!(
                "Expected next token at position {}, {} to be '{:?}', got '{:?}' instead",
                self.peek_token.get_row_position(),
                self.peek_token.get_character_position(),
                token_type,
                self.peek_token.get_type(),
            ),
        );
        self.push_error(error);
    }
}
// Adding parsing error
