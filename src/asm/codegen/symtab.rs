use std::{collections::HashMap};


pub struct SymbolTable {
    symbols: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, value: u16) {
        self.symbols.insert(name.into(), value);
    }

}