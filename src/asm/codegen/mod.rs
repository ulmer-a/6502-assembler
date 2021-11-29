mod codeblob;
mod symtab;
use std::collections::HashMap;

use self::codeblob::CodeBlob;
use super::{ldscript::LdSection, model::AsmStmt, parser::SectionSink};
use symtab::SymbolTable;

#[rustfmt::skip]
mod opcode_table;
pub use opcode_table::get_opcode;

pub struct CodeGenerator {
    sections: HashMap<String, Vec<AsmStmt>>,
    blobs: HashMap<String, CodeBlob>,
    symbols: SymbolTable,
}

impl SectionSink for CodeGenerator {
    fn push_section(&mut self, name: &str, stmts: Vec<AsmStmt>) {
        let mut stmts = stmts;
        if let Some(section_stmts) = self.sections.get_mut(name) {
            section_stmts.append(&mut stmts);
        } else {
            self.sections.insert(name.into(), stmts);
        }
    }
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            sections: HashMap::new(),
            blobs: HashMap::new(),
            symbols: SymbolTable::new_with_registers(),
        }
    }

    pub fn link(&mut self, sections_to_link: Vec<LdSection>) -> Vec<u8> {
        self.collect_symbols();
        self.generate_statements();
        self.resolve_all_symbols(&sections_to_link);

        let mut binary: Vec<u8> = vec![];
        let mut current_addr = sections_to_link[0].load_addr().unwrap();
        for section in sections_to_link.iter() {
            let load_addr = match section.load_addr() {
                Some(addr) => {
                    let padding = addr - current_addr;
                    let mut padding = vec![0u8; padding as usize];
                    binary.append(&mut padding);
                    addr
                }
                None => current_addr,
            };

            let blob = self.blobs.get_mut(section.name()).unwrap();
            let blob_size = blob.size();
            blob.dump(&mut binary);
            current_addr = load_addr + blob_size as u16;
        }
        binary
    }

    fn collect_symbols(&mut self) {
        // fill the symbol table with all constant label assignments
        // from any section so that the zeropage addr mode can be
        // used if it's available for an instruction and the address
        // fits into 8 bits.
        for (_, section_stmts) in self.sections.iter() {
            for stmt in section_stmts.iter() {
                if let AsmStmt::ConstLabel(name, addr) = stmt {
                    self.symbols.insert(name, *addr);
                }
            }
        }
    }

    fn generate_statements(&mut self) {
        for (section_name, stmts) in self.sections.iter() {
            let mut blob = CodeBlob::new();

            // iterate over all sections and statements and actually generate
            // code from the model. undefined symbols are reported for relocation.
            for stmt in stmts.iter() {
                blob.gen_stmt(stmt, |name| self.symbols.find(name));
            }

            self.blobs.insert(section_name.into(), blob);
        }
    }

    fn resolve_all_symbols(&mut self, link_sections: &Vec<LdSection>) {
        let mut current_addr = link_sections[0].load_addr().unwrap();
        for section in link_sections.iter() {
            let load_addr = match section.load_addr() {
                Some(addr) => addr,
                None => current_addr,
            };

            let blob = self.blobs.get_mut(section.name()).unwrap();
            self.symbols.insert_table(blob.symbols(), load_addr);
            current_addr = load_addr + blob.size() as u16;
        }

        self.relocate_blobs(link_sections);
    }

    fn relocate_blobs(&mut self, link_sections: &Vec<LdSection>) {
        let mut current_addr = link_sections[0].load_addr().unwrap();
        for section in link_sections.iter() {
            let load_addr = match section.load_addr() {
                Some(addr) => addr,
                None => current_addr,
            };

            // resolve symbols: go over the binary blobs again and fill in the
            // placeholders with the actual addresses that have accumulated
            // in the symbol table by now.
            let blob = self.blobs.get_mut(section.name()).unwrap();
            blob.resolve_symbols(load_addr, &self.symbols);
            current_addr = load_addr + blob.size() as u16;
        }
    }
}
