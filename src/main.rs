use interpreter::{lexer::Lexer, tokens::TokenType};


fn main() {

//     let input = "let five = (2+3);\n
// let ten = 10;\n
//         let add = fn(x, y) {\n
//         x + y;\n
//         };\n
//         let result_5_plus_10 = add(five, ten);\n
//         result_5_plus_10 == 15";

let input = "let x = 12;\n
let y = 22;\n
let is_greater = fn(a: int, b: int) {\n
    if a > b {\n
        return True;\n
    } else {\n
        return False;\n
    }\n
}\n
x != y\n
";
    let mut lexer = Lexer::new(&input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.get_type() == TokenType::EOF {break};
    }
}