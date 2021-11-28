use std::collections::HashMap;

pub struct SymbolTable {
    symbols: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: HashMap::new() }
    }

    pub fn new_with_registers() -> SymbolTable {
        let mut table = SymbolTable::new();
        for i in 0..32 {
            table.insert(&format!("r{}", i), i);
        }
        table
    }

    pub fn insert(&mut self, name: &str, value: u16) {
        self.symbols.insert(name.into(), value);
    }

    pub fn insert_table(&mut self, table: &SymbolTable, offset: u16) {
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
