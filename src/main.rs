use std::io;

use calc::parser::parser::ParseError;
use calc::parser::parser::Parser;
use calc::parser::ast;

fn main() {
    println!("calc");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("Result: {}", val),
                    Err(e) => println!("Error evaluating the expression: {:?}", e),
                }
            }
            Err(e) => println!("Error reading input: {:?}", e),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    #[cfg(debug_assertions)]
    println!("Generated AST: {:?}", ast);

    let result = ast::eval(ast)?;
    Ok(result)
}