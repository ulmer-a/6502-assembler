mod codeblob;
mod symtab;
use std::collections::HashMap;

use self::codeblob::CodeBlob;
use super::{ldscript::LdSection, model::AsmStmt, parser::SectionSink};
use symtab::SymbolTable;

#[rustfmt::skip]
mod opcode_table;

pub struct Linker {
    sections: HashMap<String, Vec<AsmStmt>>,
    blobs: HashMap<String, CodeBlob>,
    symbols: SymbolTable,
}

impl SectionSink for Linker {
    fn push_section(&mut self, name: &str, stmts: Vec<AsmStmt>) {
        let mut stmts = stmts;
        if let Some(section_stmts) = self.sections.get_mut(name) {
            section_stmts.append(&mut stmts);
        } else {
            self.sections.insert(name.into(), stmts);
        }
    }
}

impl Linker {
    pub fn new() -> Linker {
        Linker {
            sections: HashMap::new(),
            blobs: HashMap::new(),
            symbols: SymbolTable::new(),
        }
    }

    pub fn link(&mut self, sections_to_link: Vec<LdSection>) -> Vec<u8> {
        self.collect_symbols();
        self.generate_statements();
        
        let load_addr = self.calc_load_addr(sections_to_link.iter());
        println!("load_addr: 0x{:x}", load_addr);
        vec![]
    }

    fn calc_load_addr(&self, sections_to_link: std::slice::Iter<LdSection>) -> u16 {
        let mut sections_to_link = sections_to_link;
        let current_section = match sections_to_link.next() {
            Some(section) => section,
            None => return 0x0000
        };        
        match current_section.load_addr() {
            Some(addr) => addr,
            None => {
                let next_addr = self.calc_load_addr(sections_to_link);
                next_addr + self.blobs.get(current_section.name()).unwrap().size() as u16
            }
        }
    }

    fn generate_statements(&mut self) {
        for (name, obj) in self.sections.iter() {
            let mut blob = CodeBlob::new();

            for stmt in obj.iter() {
                // iterate over all sections and statements and actually generate
                // code from the model. undefined symbols are reported for relocation.
                match stmt {
                    AsmStmt::AsmInstruction(instr) => {
                        blob.gen_instruction(instr, |name| match self.symbols.find(name) {
                            Some(addr) => Some(addr),
                            None => {
                                println!("todo relocate symbol {}", name);
                                None
                            }
                        })
                    }
                    AsmStmt::Data(data) => blob.gen_data(data),
                    AsmStmt::Label(name) => blob.insert_label(name),
                    _ => {}
                }
            }

            self.blobs.insert(name.into(), blob);
        }
    }

    fn collect_symbols(&mut self) {
        // fill the symbol table with all constant label assignments
        // from any section so that the zeropage addr mode can be
        // used if it's available for an instruction and the address
        // fits into 8 bits.
        for (_, obj) in self.sections.iter() {
            for stmt in obj.iter() {
                if let AsmStmt::ConstLabel(name, addr) = stmt {
                    self.symbols.insert(name, *addr);
                }
            }
        }
    }
}
