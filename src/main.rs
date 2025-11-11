mod compiler;
mod lexer;
mod parser;
mod semantic;

use compiler::LolCompiler;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input_file.lol>");
        process::exit(1);
    }

    let input_file = &args[1];

    if !input_file.ends_with(".lol") {
        eprintln!("Error: Input file must have .lol extension");
        process::exit(1);
    }

    let source = std::fs::read_to_string(input_file).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", input_file, err);
        process::exit(1);
    });

    let mut compiler = LolCompiler::new();
    compiler.compile_file(&source, input_file);

    println!(
        "Compilation successful! Output written to {}",
        input_file.replace(".lol", ".html")
    );
}
