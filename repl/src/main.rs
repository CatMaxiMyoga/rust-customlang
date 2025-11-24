#![allow(missing_docs)]

use lexer::{Lexer, types::Token};
use parser::{Parser, types::Program};
use interpreter::{Interpreter, types::Environment};
use std::io::{self, Write};

fn main() {
    let mut environment: Environment = Environment::new();

    loop {
        print!("\n>> ");
        let _ = io::stdout().flush();
        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        input = input[..input.len().saturating_sub(1)].to_string(); // Remove newline
        let tokens: Result<Vec<Token>, String> = Lexer::tokenize(&input);

        match tokens {
            Ok(toks) => {
                println!("\nTokens:");
                for tok in &toks {
                    println!("{tok:?}");
                }

                let ast: Result<Program, String> = Parser::parse(toks);

                match ast {
                    Ok(program) => {
                        println!("\nAST:");
                        for stmt in &program.statements {
                            println!("{stmt:?}");
                        }

                        println!("\nInterpreter Output:");
                        Interpreter::run(program, &mut environment).unwrap_or_else(|e| {
                            println!("Interpreter error: {e:?}");
                        });
                    }
                    Err(e) => {
                        println!("Parser error: {e}");
                    }
                }
            }
            Err(e) => {
                println!("Lexer error: {e}");
            }
        }
    }
}
