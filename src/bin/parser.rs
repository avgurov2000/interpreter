use ::interpreter::{lexer::Lexer, parsing::Parser};

fn main() {
    let input = "let x = 5;\n
return x;\n
let = 10;\n
x\n
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse();

    let errors = parser.get_errors();
    for i in errors {
        println!("{:?}", i.get_message());
    }

    // if let Err(msg) = program {
    //     panic!("Program parsing returned error {:?}", msg);
    // } else if program.as_ref().unwrap().len() != 2 {
    //     panic!(
    //         "program statements must contain 3 elements, got {}",
    //         program.as_ref().unwrap().len()
    //     );
    // }

    let program = program.unwrap();

    for idx in 0..program.len() {
        let statement = program.get_item(idx);
        println!("{:?}", statement.get_string())
    }
}
