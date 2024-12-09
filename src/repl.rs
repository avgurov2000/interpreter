use super::{lexer::Lexer, tokens::TokenType};
use std::io::{BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut scanner = input.lines();

    let mut line_count = 0;
    loop {
        // Write the prompt
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        if let Some(Ok(line)) = scanner.next() {
            let mut lexer = Lexer::new(&line);
            lexer.set_file_position(line_count, 0);
            loop {
                let token = lexer.next_token();
                if token.get_type() == TokenType::EOF {
                    break;
                }
                writeln!(output, "{:?}", token).unwrap();
            }
        } else {
            break;
        }
        line_count += 1;
    }
}
