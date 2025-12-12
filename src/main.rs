use std::io::{self, Write};

use crate::{errors::CalculatorError, evaluator::Evaluator, lexer::Lexer, output::Value, parser::Parser};

mod ast;
mod errors;
mod evaluator;
mod lexer;
mod parser;
mod output;

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
            Ok(_) => {}
            Err(_) => {
                panic!("Error reading stdin");
            }
        }

        if operation.trim() == "exit" {
            break;
        }

        let mut lexer = Lexer::new(&operation);

        let tokens = lexer.tokenize();

        let tokens = match tokens {
            Ok(tokens) => tokens,
            Err(error) => match error {
                CalculatorError::InvalidCharacter(c) => {
                    println!("Invalid character: {}", c);
                    continue;
                }
                CalculatorError::InvalidNumberFormat => {
                    println!("Invalid number format");
                    continue;
                }
                _ => unreachable!(),
            },
        };

        let mut parser = Parser::new(tokens);

        let parent_node = match parser.parse_input() {
            Ok(node) => node,
            Err(_) => {
                println!("error");
                continue;
            }
        };

        let result = match evaluator.evaluate(&parent_node) {
            Ok(result) => result,
            Err(_) => {
                println!("errorp");
                continue;
            }
        };

        match result {
            Value::Number(n) => println!("{}", n),
            Value::Boolean(b) => println!("{}", b),
        }
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
            let mut parser = Parser::new(lexer.tokenize().unwrap());

            let result_node = parser.parse_expression().unwrap();

            let result = evaluator.evaluate(&result_node).unwrap();

            assert_eq!(result.as_number().unwrap(), 19.3);
        }

        {
            let operation = String::from("-5 + 3");
            let mut evaluator = Evaluator::new();

            let mut lexer = Lexer::new(&operation);
            let tokens = lexer.tokenize();
            let mut parser = Parser::new(tokens.unwrap());

            let result_node = parser.parse_expression().unwrap();

            let result = evaluator.evaluate(&result_node).unwrap();

            assert_eq!(result.as_number().unwrap(), -2.0);
        }
    }
}
