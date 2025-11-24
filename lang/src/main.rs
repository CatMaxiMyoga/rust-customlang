#![allow(missing_docs)]

use std::path::Path;

use interpreter::{Interpreter, types::{Environment, RuntimeError}};
use lexer::{Lexer, types::Token};
use parser::{Parser, types::Program};

const LANGUAGE_EXTENSION: &str = "custom";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Usage: lang <source-file>");
        std::process::exit(1);
    }

    let filename: &str = &args[0];
    let filepath: &Path = Path::new(filename);
    if !filepath.exists() {
        eprintln!("File not found: {filename}");
        std::process::exit(1);
    }

    if let Some(extension) = filepath.extension() {
        if extension != LANGUAGE_EXTENSION {
            eprintln!("Invalid file extension. Expected a .cl file.");
            std::process::exit(1);
        }
    } else {
        eprintln!("Unable to read file extension. Expected a .cl file.");
        std::process::exit(1);
    }

    let source_code: String = std::fs::read_to_string(filepath).unwrap_or_else(|e| {
        eprintln!("Error reading file: {e}");
        std::process::exit(1);
    });

    let tokens: Result<Vec<Token>, String> = Lexer::tokenize(&source_code);
    let tokens: Vec<Token> = match tokens {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {e}");
            std::process::exit(1);
        },
    };

    let program: Result<Program, String> = Parser::parse(tokens);
    let program: Program = match program {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {e}");
            std::process::exit(1);
        },
    };

    let mut environment: Environment = Environment::new();

    let runtime_result: Result<(), RuntimeError> = Interpreter::run(program, &mut environment);
    if let Err(e) = runtime_result {
        eprintln!("RuntimeError: {e:?}");
        std::process::exit(1);
    }
}
