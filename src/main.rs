use std::{env, fs};

mod lexer;
use lexer::{AsmLexer};

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let source = fs::read_to_string(filename).unwrap();

    let mut lexer = AsmLexer::new(&source);
    lexer.next_token();
}
