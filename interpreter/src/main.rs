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
        let mut lx: Lexer = Lexer::new(input);
        let tokens: Result<Vec<Token>, String> = lx.tokenize();

        match tokens {
            Ok(toks) => {
                for tok in toks {
                    println!("{:?}", tok);
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
