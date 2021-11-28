use std::collections::HashMap;

use super::SectionSink;
use crate::asm::model::AsmStmt;

mod instruction_parse_tests;
mod parse_tests;
mod section_parse_tests;

struct StmtCollector {
    stmts: HashMap<String, Vec<AsmStmt>>,
}

impl StmtCollector {
    pub fn new() -> StmtCollector {
        StmtCollector {
            stmts: HashMap::new(),
        }
    }

    pub fn statements(&self) -> &Vec<AsmStmt> {
        self.section_statements("text")
    }

    pub fn section_statements(&self, name: &str) -> &Vec<AsmStmt> {
        self.stmts.get(name).unwrap()
    }
}

impl SectionSink for StmtCollector {
    fn push_section(&mut self, name: &str, stmts: Vec<AsmStmt>) {
        let mut stmts = stmts;
        if let Some(section_stmts) = self.stmts.get_mut(name) {
            section_stmts.append(&mut stmts);
        } else {
            self.stmts.insert(name.into(), stmts);
        }
    }
}
