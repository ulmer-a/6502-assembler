use std::collections::HashMap;

use super::{opcode_table::get_opcode, symtab::SymbolTable};
use crate::asm::model::{AddrMode, AsmStmt, DataPlacement, IndexMode, Instruction, MemRef};

pub struct CodeBlob {
    blob: Vec<u8>,
    symbols: SymbolTable,
    rel8: HashMap<String, u16>,
    rel16: HashMap<String, u16>,
}

impl CodeBlob {
    pub fn new() -> CodeBlob {
        CodeBlob {
            blob: vec![],
            symbols: SymbolTable::new(),
            rel8: HashMap::new(),
            rel16: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.blob.len()
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    pub fn dump(&mut self, binary: &mut Vec<u8>) {
        binary.append(&mut self.blob);
    }

    pub fn resolve_symbols(&mut self, base_addr: u16, global_symbols: &SymbolTable) {
        for (name, offset) in self.rel16.iter() {
            match global_symbols.find(name) {
                Some(addr) => {
                    self.blob[*offset as usize] = (addr & 0xff) as u8;
                    self.blob[*offset as usize + 1] = (addr >> 8) as u8;
                }
                None => {
                    println!("undefined reference to symbol {}", name);
                    continue;
                }
            }
        }
        for (name, offset) in self.rel8.iter() {
            if let Some(addr) = global_symbols.find(name) {
                let delta = (addr as i16 - (base_addr + offset) as i16) as i8;
                self.blob[*offset as usize] = delta as u8;
            } else {
                println!("undefined reference to symbol {}", name);
                continue;
            }
        }
    }

    pub fn gen_stmt<F>(&mut self, stmt: &AsmStmt, symbol_lookup: F)
    where
        F: Fn(&str) -> Option<u16>,
    {
        match stmt {
            AsmStmt::AsmInstruction(instr) => self.gen_instruction(instr, symbol_lookup),
            AsmStmt::Data(data) => self.gen_data(data),
            AsmStmt::Label(name) => self.insert_label(name),
            _ => {}
        }
    }

    pub fn insert_label(&mut self, name: &str) {
        let current_addr = self.blob.len();
        assert!(current_addr <= 0xffff);
        self.symbols.insert(name, current_addr as u16);
    }

    pub fn gen_data(&mut self, data: &DataPlacement) {
        match data {
            DataPlacement::Str(string) => {
                let mut bytes = string.clone().into_bytes();
                bytes.push(0x00);
                self.blob.append(&mut bytes);
            }
        }
    }

    pub fn gen_instruction<F>(&mut self, instruction: &Instruction, lookup: F)
    where
        F: Fn(&str) -> Option<u16>,
    {
        fn addr_to_vector(addr: u16) -> Vec<u8> {
            vec![(addr >> 8) as u8, (addr & 0xff) as u8]
        }

        let mnemonic_i = instruction.mnemonic_index();
        let (addr_mode_i, ref mut operand) = match instruction.addr_mode() {
            AddrMode::Implied => (0, vec![]),
            AddrMode::Immediate(addr) => (1, vec![addr]),
            AddrMode::Memory(mode, mem_ref) => {
                let addr = match mem_ref {
                    MemRef::Addr(addr) => Some(addr),
                    MemRef::Variable(name) => {
                        let addr = lookup(&name);
                        if addr.is_none() {
                            let rel_addr = (self.blob.len() + 1) as u16;
                            if instruction.has_rel_addressing() {
                                self.rel8.insert(name, rel_addr);
                            } else {
                                self.rel16.insert(name, rel_addr);
                            }
                        }
                        addr
                    }
                };

                if addr.is_some() && addr.unwrap() < 256 {
                    let addr = addr.unwrap();
                    match mode {
                        IndexMode::None => (2, vec![addr as u8]),
                        IndexMode::IndexedX => (3, vec![addr as u8]),
                        IndexMode::IndexedY => (4, vec![addr as u8]),
                    }
                } else {
                    match mode {
                        IndexMode::None => (8, addr_to_vector(0)),
                        IndexMode::IndexedX => (9, addr_to_vector(0)),
                        IndexMode::IndexedY => (10, addr_to_vector(0)),
                    }
                }
            }
        };

        if let Some(opcode) = get_opcode(mnemonic_i, addr_mode_i) {
            self.blob.push(opcode);
            self.blob.append(operand);
        } else {
            println!("invalid addr mode {:?}", instruction.addr_mode());
        }
    }
}
