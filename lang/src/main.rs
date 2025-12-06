#![allow(missing_docs)]

use std::path::Path;

use compiler::Compiler;
use lexer::{Lexer, types::Token};
use parser::{Parser, types::Program};

const LANGUAGE_EXTENSION: &str = "custom";

const USAGE: &str = r"
USAGE: lang <source-file> [OPTIONS] [-- GCC-OPTIONS]

OPTIONS:
    -t              Transpile only, do not compile.
    -o <file>       Specify output file name.
    -C              Cleans up the out/ directory before compilation.
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

    let mut clean_up: bool = false;
    let mut transpile_only: bool = false;
    let mut out_file: String = String::new();
    let mut gcc_args: Vec<String> = Vec::new();

    while !args.is_empty() {
        let arg: String = args.remove(0);
        match arg.as_str() {
            "-C" => clean_up = true,
            "-t" => transpile_only = true,
            "-tC" | "-Ct" => {
                transpile_only = true;
                clean_up = true;
            }
            "-o" => {
                if args.is_empty() {
                    eprint!("Expected output file name after -o {USAGE}");
                    std::process::exit(1);
                }
                out_file = args.remove(0);
            }
            "--" => {
                if args.contains(&String::from("-o")) {
                    eprint!("Cannot specify output file (-o <file>) in GCC options. {USAGE}");
                    std::process::exit(1);
                }

                gcc_args = args
                    .into_iter()
                    .map(|x| format!(r#""{}""#, x.replace('"', "\\\"")))
                    .collect();
                break;
            }
            _ => {
                eprint!("Unknown option: {arg}");
                eprint!("{USAGE}");
                std::process::exit(1);
            }
        }
    }

    if transpile_only && !out_file.is_empty() {
        eprintln!("Cannot specify output file (-o <file>) when using transpile-only flag (-t)");
        std::process::exit(1);
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

    if let Err(err) = Compiler::compile(program, &out_file, transpile_only, &gcc_args, clean_up) {
        eprintln!("Compiler error: {err}");
        std::process::exit(1);
    }
}
