use std::{env, fs};
mod asm_model;
mod asm_parser;
mod lexer;
use asm_parser::AsmParser;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let source = fs::read_to_string(filename).unwrap();

    let mut parser = AsmParser::new(&source);
    parser.parse();
}
