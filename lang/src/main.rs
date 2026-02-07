#![allow(missing_docs)]

use std::path::Path;

use compiler::Compiler;
use lexer::{Lexer, types::Token};
use parser::{Parser, types::Program};
use transpiler::Transpiler;

const LANGUAGE_EXTENSION: &str = "custom";

const USAGE: &str = r"

USAGE:
  lang <source-file> [OPTIONS]
  lang -h

OPTIONS:
  -h  --help              Show this help message. The only option that does not require a
                           source file to be specified.
  -o <output-file>        Specify the output file name/path. Accepts both relative and
                           absolute paths.
  -s  --step <0-3>        Specify the compilation step to stop after. If provided and
                           greater than 0, output of the given step will be output to
                           stdout. Uses rust's debug formatting (:?) for steps 1 and 2,
                           or rust's pretty debug formatting (:#?) if -p/--pretty is also
                           specified. Cannot be used with -o when step is greater than 0.
                             0: All steps (default)
                             1: Lexical Analysis / Tokenization
                             2: Parsing / AST Generation
                             3: Transpilation
  -p  --pretty            Pretty-print the output when using -s/--step with a value
                           of either 1 or 2. Not allowed otherwised
";

#[allow(clippy::too_many_lines)]
fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprint!("{USAGE}");
        std::process::exit(1);
    }

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print!("{USAGE}");
        std::process::exit(0);
    }

    let filename: String = args.remove(0);
    let filepath: &Path = Path::new(&filename);
    if !filepath.exists() {
        eprint!("File not found: {filename} {USAGE}");
        std::process::exit(1);
    }

    let mut output_filename: Option<String> = None;

    if let Some(index) = args.iter().position(|x| x == "-o") {
        if index + 1 >= args.len() {
            eprint!("Missing value for -o option. {USAGE}");
            std::process::exit(1);
        }

        output_filename = Some(args[index + 1].clone());
        args.drain(index..=index + 1);
    }

    let step: u8 = args
        .iter()
        .position(|x| x == "-s" || x == "--step")
        .map_or(0, |index| {
            if index + 1 >= args.len() {
                eprint!("Missing value for -s/--step option. {USAGE}");
                std::process::exit(1);
            }

            let step_str: String = args[index + 1].clone();
            let step_parsed: u8 = match step_str.parse() {
                Ok(num) if num <= 3 && num > 0 && output_filename.is_some() => {
                    eprint!(
                        "-s/--step cannot be used with -o when step is greater than 0. {USAGE}"
                    );
                    std::process::exit(1);
                }
                Ok(num) if num <= 3 => num,
                _ => {
                    eprint!(
                        "Invalid value for -s/--step. Must be integer between 0 and 3. {USAGE}"
                    );
                    std::process::exit(1);
                }
            };

            args.drain(index..=index + 1);

            step_parsed
        });

    let pretty: bool = args.iter().any(|x| x == "-p" || x == "--pretty");

    if pretty && !(step == 1 || step == 2) {
        eprint!("-p/--pretty can only be used with -s/--step when step is 1 or 2. {USAGE}");
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

    if step == 1 {
        if pretty {
            print!("{tokens:#?}");
        } else {
            print!("{tokens:?}");
        }
        std::process::exit(0);
    }

    let program: Result<Program, String> = Parser::parse(tokens);
    let program: Program = match program {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {e}");
            std::process::exit(1);
        }
    };

    if step == 2 {
        if pretty {
            print!("{program:#?}");
        } else {
            print!("{program:?}");
        }
        std::process::exit(0);
    }

    let transpiled_code: Result<String, String> = Transpiler::transpile(program);

    let transpiled_code: String = match transpiled_code {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Transpiler error: {e}");
            std::process::exit(1);
        }
    };

    if step == 3 {
        print!("{transpiled_code}");
        std::process::exit(0);
    }

    Compiler::compile(&transpiled_code, output_filename);
}
