use std::io;
use calculator::parsemath::parser::Parser;
use calculator::parsemath::ast::eval;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    loop {
        println!("Please enter the arithmetic expression: ");
        let mut inp_buf = String::new();
        match io::stdin().read_line(&mut inp_buf) {
            Ok(_) => {
                // 1. generate the parser
                let mut parser = match Parser::new(&inp_buf) {
                    Ok(p) => p, 
                    Err(e) => { 
                        println!("fail to create new parser: {}", e);
                        continue;
                    }
                };

                // 2. generate the AST
                let ast = match parser.parse() {
                    Ok(node) => node,
                    Err(e) => {
                        println!("fail to parse the expression: {}", e);
                        continue;
                    },
                };
                
                // 3. evaluate the result
                let result = match eval(&ast) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("fail to evaluate the AST {:?}: {}", &ast, e);
                        continue;
                    }
                };
                println!("The evaluation result is {}", result);
            },

            Err(e) => println!("Fail to read from stdin: {}", e),
        }
    }
}
