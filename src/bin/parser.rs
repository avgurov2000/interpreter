use ::interpreter::{ast::LetStatement, lexer::Lexer, parser::Parser};
use interpreter::ast::Node;

fn main() {
    let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

    let mut lexer = Lexer::new(&input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse();

    if let Err(msg) = program {
        panic!("Program parsing returned error {:?}", msg);
    } else if program.as_ref().unwrap().len() != 3 {
        panic!(
            "program statements must contain 3 elements, got {}",
            program.as_ref().unwrap().len()
        );
    }

    let program = program.unwrap();
    let expected_literals = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];

    for (idx, expected_literal) in expected_literals.into_iter().enumerate() {
        let statement = program.get_item(idx).as_ref();
        let let_statement = statement.as_any().downcast_ref::<LetStatement>().unwrap();
        println!("{:?}", let_statement.name().token_literal())
    }
}
