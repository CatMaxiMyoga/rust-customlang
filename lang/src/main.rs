#![allow(missing_docs)]

use std::path::Path;

use compiler::Compiler;
use lexer::{Lexer, types::Token};
use parser::{Parser, types::Program};
use transpiler::Transpiler;

const LANGUAGE_EXTENSION: &str = "custom";

const USAGE: &str = r"
USAGE: lang <source-file> [-o <output_file>]
";

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprint!("{USAGE}");
        std::process::exit(1);
    }

    let filename: String = args.remove(0);
    let filepath: &Path = Path::new(&filename);
    if !filepath.exists() {
        eprint!("File not found: {filename} {USAGE}");
        std::process::exit(1);
    }

    let mut output_filename: Option<String> = None;

    if args.len() > 2 {
        let index: usize = args.iter().position(|x| x == "-o").unwrap_or(usize::MAX);

        if index != usize::MAX && index < args.len() {
            eprintln!("Invalid number of arguments. {USAGE}");
            std::process::exit(1);
        }

        if index != usize::MAX {
            output_filename = Some(args[(index + 1) as usize].clone());
            args.drain(index as usize..=index as usize + 1);
        }
    }

    if let Some(extension) = filepath.extension() {
        if extension != LANGUAGE_EXTENSION {
            eprintln!("Invalid file extension. Expected a .{LANGUAGE_EXTENSION} file.");
            std::process::exit(1);
        }
    } else {
        eprintln!("Unable to read file extension. Expected a .{LANGUAGE_EXTENSION} file.");
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
        }
    };

    let program: Result<Program, String> = Parser::parse(tokens);
    let program: Program = match program {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {e}");
            std::process::exit(1);
        }
    };

    let transpiled_code: String = Transpiler::transpile(program);

    Compiler::compile(&transpiled_code, output_filename);
}
