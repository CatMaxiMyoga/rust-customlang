use parser::{Parser, types::Program};

/// This is temporary code for testing purposes.
fn repl() {
    use lexer::{Lexer, types::Token};
    use std::io::{self, Write};

    loop {
        // Get user input
        print!(">> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        input = input[..input.len().saturating_sub(1)].to_string(); // Remove newline
        let mut l: Lexer = Lexer::new(input);
        let tokens: Result<Vec<Token>, String> = l.tokenize();

        match tokens {
            Ok(toks) => {
                println!("\nTokens:");
                for tok in toks.iter() {
                    println!("{:?}", tok);
                }

                let mut p: Parser = Parser::new(toks);
                let ast: Result<Program, String> = p.parse();

                match ast {
                    Ok(program) => {
                        println!("\nAST:");
                        for stmt in program.statements.iter() {
                            println!("{:?}", stmt);
                        }
                    }
                    Err(e) => {
                        println!("Parser error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Lexer error: {}", e);
            }
        }
    }
}

fn main() {
    repl();
}
