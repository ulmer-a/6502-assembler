use std::{env, fs, io::Write};
mod asm;
mod errors;
use asm::{AsmParser, CodeGenerator};

use crate::asm::ldscript::LdSection;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let source = fs::read_to_string(filename).unwrap();

    let mut linker = CodeGenerator::new();
    let mut parser = AsmParser::new(&source);
    parser.parse(&mut linker);

    if parser.dump_errors() == 0 {
        let ldscript = vec![
            LdSection::new("text", Some(0xe000)),
            LdSection::new("data", None),
        ];
        let binary = linker.link(ldscript);
        let mut file = fs::File::create("output.bin").unwrap();
        file.write(&binary).unwrap();
    }
}
