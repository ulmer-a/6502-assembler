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

    pub fn insert(&mut self, name: &str, value: u16) {
        self.symbols.insert(name.into(), value);
    }

    pub fn find(&self, name: &str) -> Option<u16> {
        match self.symbols.get(name.into()) {
            Some(addr) => Some(*addr),
            None => None,
        }
    }
}
