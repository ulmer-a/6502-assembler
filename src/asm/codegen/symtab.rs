use std::collections::HashMap;

pub struct SymbolTable {
    symbols: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbols = HashMap::new();
        for i in 0..32 {
            symbols.insert(format!("r{}", i), i);
        }
        SymbolTable { symbols }
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
