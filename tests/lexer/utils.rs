#[cfg(test)]
pub mod utils_fn {

    use std::sync::Once;

    use env_logger;
    use interpreter::tokens::{Token, TokenType};

    pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

    static INIT: Once = Once::new();
    pub fn init_logger() {
        INIT.call_once(|| {
            env_logger::builder()
                .is_test(true) // Simplifies log formatting for tests
                .init();
        });
    }

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
