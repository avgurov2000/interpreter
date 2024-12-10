#[cfg(test)]
pub mod utils_fn {

    use interpreter::tokens::{Token, TokenType};

    pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

    pub fn create_token_full(
        token_type: TokenType,
        ch: Option<String>,
        row_position: usize,
        characher_position: usize,
    ) -> Token {
        Token::new(token_type, ch, row_position, characher_position)
    }

    pub fn create_token_short(token_type: TokenType, ch: Option<String>) -> Token {
        Token::new(token_type, ch, 0, 0)
    }
}
