#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum DataType {
    Bool,
    Char,
    Float,
    Int,
    None,
    String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum PunctuationType {
    Ampersand,   // &
    Assign,      // =
    Asterisk,    // *
    BackSlash,   // \
    Colon,       // :
    Comma,       // ,
    DoubleSlash, // //
    Equal,       // ==,
    Exclamation, // !
    Greater,     // >
    LBrace,      // {
    LParen,      // (
    Less,        // <
    Minus,       // -
    NotEqual,    // !=
    Plus,        // +
    Question,    // ?
    RBrace,      // }
    RParen,      // )
    Semicolon,   // ;
    Slash,       // /
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ControlFlowType {
    Else,
    If,
    Switch,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum SpecialWordType {
    False,
    Function,
    Let,
    Return,
    True,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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
    ch: Option<String>,
    position: (usize, usize),
}

impl Token {
    pub fn new(
        token_type: TokenType,
        ch: Option<String>,
        row_position: usize,
        character_position: usize,
    ) -> Self {
        Token {
            token_type,
            ch,
            position: (row_position, character_position),
        }
    }
    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }
    pub fn get_ch(&self) -> Option<String> {
        self.ch.clone()
    }

    pub fn get_row_position(&self) -> usize {
        self.position.0
    }

    pub fn get_character_position(&self) -> usize {
        self.position.1
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token::new(
            self.get_type(),
            self.get_ch(),
            self.get_row_position(),
            self.get_character_position(),
        )
    }
}
