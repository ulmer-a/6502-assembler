use std::collections::HashMap;

pub struct SymbolTable {
    symbols: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn new_with_registers() -> SymbolTable {
        let mut table = SymbolTable::new();
        for i in 0..32 {
            // insert pseudo register symbols r0..r31
            table.insert(&format!("r{}", i), i);
        }
        table
    }

    pub fn insert(&mut self, name: &str, value: u16) {
        self.symbols.insert(name.into(), value);
    }

    pub fn insert_table(&mut self, table: &SymbolTable, offset: u16) {
        // merge with another symbol table object
        for (name, addr) in table.symbols.iter() {
            self.symbols.insert(name.into(), addr + offset);
        }
    }

    pub fn find(&self, name: &str) -> Option<u16> {
        match self.symbols.get(name.into()) {
            Some(addr) => Some(*addr),
            None => None,
        }
    }
}

#[test]
fn symtab_simple() {
    let mut symbols = SymbolTable::new();
    symbols.insert("my_func", 0x8abc);
    assert_eq!(symbols.find("my_func"), Some(0x8abc));
}

#[test]
fn symtab_merge() {
    let mut symbols1 = SymbolTable::new();
    symbols1.insert("my_func", 0x8abc);

    let mut symbols2 = SymbolTable::new();
    symbols2.insert_table(&symbols1, 0xcde);

    assert_eq!(symbols2.find("my_func"), Some(0x8abc + 0xcde));
}

#[test]
fn symtab_pseudo_registers() {
    let symbols1 = SymbolTable::new();
    let symbols2 = SymbolTable::new_with_registers();

    assert_eq!(symbols1.find("r14"), None);
    assert_eq!(symbols2.find("r14"), Some(14));
}
