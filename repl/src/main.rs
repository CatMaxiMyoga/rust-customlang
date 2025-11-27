#![allow(missing_docs)]

use interpreter::{Interpreter, types::Scope};
use lexer::{Lexer, types::Token};
use parser::Parser;
use std::io::{self, Write};

fn main() {
    let mut environment: Scope = Scope::default();
    let mut buffer: String = String::new();

    loop {
        if buffer.is_empty() {
            print!(">>> ");
        } else {
            print!("... ");
        }

        let _ = io::stdout().flush();

        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line.");
            continue;
        }

        buffer.push_str(&input);

        let tokens: Vec<Token> = match Lexer::tokenize(&buffer) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Lexer error: {e}");
                buffer.clear();
                continue;
            }
        };

        match Parser::parse(tokens) {
            Ok(program) => {
                Interpreter::run(program, &mut environment).unwrap_or_else(|e| {
                    eprintln!("Interpreter error: {e:?}");
                });
                buffer.clear();
            }
            Err(e) => {
                let msg: String = e.to_lowercase();

                if msg.ends_with("end of input")
                    || msg.ends_with("endoffile")
                    || msg.ends_with("'endoffile'")
                {
                    continue;
                }

                eprintln!("Parser error: {e}");
                buffer.clear();
            }
        }
    }
}
