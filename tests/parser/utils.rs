#[cfg(test)]
pub mod utils_fn {
    use interpreter::parsing::Parser;

    pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

    pub fn check_parsing_error(parser: &Parser) -> bool {
        if parser.get_errors().is_empty() {
            false
        } else {
            for err in parser.get_errors() {
                eprintln!("Found parsing error: \n{}\n", err);
            }
            true
        }
    }
}
