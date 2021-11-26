mod symtab;
use symtab::SymbolTable;
use super::model::AsmStmt;

pub struct Linker<'a> {
    objects: Vec<&'a Vec<AsmStmt>>,
    symbols: SymbolTable,
}

impl<'a> Linker<'a> {
    pub fn new() -> Linker<'a> {
        Linker {
            objects: vec![],
            symbols: SymbolTable::new(),
        }
    }

    pub fn add_obj(&mut self, obj: &'a Vec<AsmStmt>) {
        self.objects.push(obj);
    }

    pub fn link(&mut self) {
        self.collect_symbols();
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