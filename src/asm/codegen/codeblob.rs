use super::{opcode_table::OPCODE_TABLE, symtab::SymbolTable};
use crate::asm::model::{AddrMode, IndexMode, Instruction, MemRef};

pub struct CodeBlob {
    blob: Vec<u8>,
    symbols: SymbolTable,
}

impl CodeBlob {
    pub fn new() -> CodeBlob {
        CodeBlob {
            blob: vec![],
            symbols: SymbolTable::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.blob.len()
    }

    pub fn insert_label(&mut self, name: &str) {
        let current_addr = self.blob.len();
        assert!(current_addr <= 0xffff);
        self.symbols.insert(name, current_addr as u16);
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
                    MemRef::Variable(name) => lookup(&name),
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

        if let Some(opcode) = Self::get_opcode(mnemonic_i, addr_mode_i) {
            self.blob.push(opcode);
            self.blob.append(operand);
        } else {
            println!("invalid addr mode {:?}", instruction.addr_mode());
        }
    }

    fn get_opcode(mnemonic_i: usize, addr_mode_i: usize) -> Option<u8> {
        match OPCODE_TABLE[mnemonic_i][addr_mode_i] {
            -1 => Self::get_opcode(mnemonic_i, 13),
            opcode => Some(opcode as u8),
        }
    }
}
