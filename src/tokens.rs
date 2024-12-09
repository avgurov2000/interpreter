
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Bool,
    Char,
    Float,
    Int,
    None,
    String,
}


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationType {
    Ampersand, // &
    Assign, // =
    Asterisk, // *
    BackSlash, // \
    Colon, // :
    Comma, // ,
    Greater, // >
    LBrace, // {
    LParen, // (
    Less, // <
    Minus, // -
    Plus, // +
    RBrace, // }
    RParen, // )
    Semicolon, // ;
    Slash, // /
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlowType {
    Else,
    If,
    Switch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialWordType {
    False, 
    Function, 
    Let,
    Return,
    True, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ControlFlow(ControlFlowType),
    Data(DataType), 
    EOF,
    Ident,
    Illegal,
    Punctuation(PunctuationType),
    SpecialWord(SpecialWordType),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    ch: Option::<String>,
}

impl Token {
    pub fn new(token_type: TokenType, ch: Option::<String>) -> Self {
        Token {token_type, ch}
    }
    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }
    pub fn get_ch(&self) -> Option::<String> {
        if let Some(i) = &self.ch {
            Some(i.clone())
        } else {
            None
        }
    }
}

