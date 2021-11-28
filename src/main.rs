use std::{env, fs};
mod asm;
mod errors;
use asm::{AsmParser, Linker};

use crate::asm::ldscript::LdSection;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let source = fs::read_to_string(filename).unwrap();

    let mut linker = Linker::new();
    let mut parser = AsmParser::new(&source);
    parser.parse(&mut linker);

    if parser.dump_errors() == 0 {
        let ldscript = vec![
            LdSection::new("text", Some(0xe000)),
            LdSection::new("data", None),
        ];
        linker.link(ldscript);
    }
}
