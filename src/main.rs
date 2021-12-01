use std::{env, fs, io::Write};
mod asm;
mod errors;
use asm::{AsmParser, CodeGenerator};

use crate::asm::ldscript::LdSection;

fn main() {
    let mut codegen = CodeGenerator::new();
    for filename in env::args().skip(1) {
        let source = match fs::read_to_string(&filename) {
            Ok(source) => source,
            Err(_) => {
                println!("error: {}: cannot open file", &filename);
                return;
            }
        };

        let mut parser = AsmParser::new(&source);
        parser.parse(&mut codegen);

        if parser.dump_errors() != 0 {
            return;
        }
    }

    let ldscript = vec![
        LdSection::new("text", Some(0xe000)),
        LdSection::new("data", None),
    ];

    match codegen.link(ldscript) {
        Ok(binary) => {
            let mut file = fs::File::create("output.bin").unwrap();
            file.write(&binary).unwrap();
        },
        Err(errors) => {
            for error in errors {
                println!("codegen error: {}", error);
            }
        }
    }
}
