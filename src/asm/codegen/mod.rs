mod codeblob;
mod symtab;
use self::codeblob::CodeBlob;
use super::model::AsmStmt;
use symtab::SymbolTable;

#[rustfmt::skip]
mod opcode_table;

pub struct Linker<'a> {
    objects: Vec<&'a Vec<AsmStmt>>,
    symbols: SymbolTable,
    blob: CodeBlob,
}

impl<'a> Linker<'a> {
    pub fn new() -> Linker<'a> {
        Linker {
            objects: vec![],
            symbols: SymbolTable::new(),
            blob: CodeBlob::new(),
        }
    }

    pub fn add_obj(&mut self, obj: &'a Vec<AsmStmt>) {
        self.objects.push(obj);
    }

    pub fn link(&mut self) {
        self.collect_symbols();

        for obj in self.objects.iter() {
            for stmt in obj.iter() {
                if let AsmStmt::AsmInstruction(instr) = stmt {
                    self.blob
                        .gen_instruction(instr, |name| match self.symbols.find(name) {
                            Some(addr) => addr,
                            None => {
                                println!("todo relocate symbol {}", name);
                                0x0000
                            }
                        });
                }
            }
        }
    }

    fn collect_symbols(&mut self) {
        for obj in self.objects.iter() {
            for stmt in *obj {
                if let AsmStmt::ConstLabel(name, addr) = stmt {
                    self.symbols.insert(name, *addr);
                }
            }
        }
    }
}
