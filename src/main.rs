use std::io::{self, Write};

use crate::{
    evaluator::Evaluator, lexer::{Lexer}, parser::Parser
};

mod ast;
mod lexer;
mod parser;
mod evaluator;

fn main() {
    println!("***** Simple calculator *****");
    println!("Enter your operations (type 'exit' to leave)");
    
    let mut operation = String::from("");

    let mut evaluator = Evaluator::new();
    
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush the output");
        
        operation.clear();

        match io::stdin().read_line(&mut operation) {
            Ok(_) => {},
            Err(_) => {
                panic!("Error reading stdin");
            }
        }

        if operation.trim() == "exit" {
            break;
        }

        let mut lexer = Lexer::new(&operation);
        let mut parser = Parser::new(lexer.tokenize());
        let result = evaluator.evaluate(&parser.parse_input());

        println!("{}", result);
    }
}

#[cfg(test)]
mod test {
    use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

    #[test]
    fn test_evaulate() {
        {
            let operation = String::from("12.5 + 3 * (4.2 / 2) - (7 / 2) + 6.0 / 3 * 2");

            let mut evaluator = Evaluator::new();

            let mut lexer = Lexer::new(&operation);
            let mut parser = Parser::new(lexer.tokenize());

            let result_node = parser.parse_expression();

            let result = evaluator.evaluate(&result_node);

            assert_eq!(result, 19.3);
        }

        {
            let operation = String::from("-5 + 3");
            let mut evaluator = Evaluator::new();

            let mut lexer = Lexer::new(&operation);
            let tokens = lexer.tokenize();
            let mut parser = Parser::new(tokens);

            let result_node = parser.parse_expression();

            let result = evaluator.evaluate(&result_node);
            

            assert_eq!(result, -2.0);
        }
    }
}
