use crate::asm::model::AsmStmt;

use super::SectionSink;

mod instruction_parse_tests;
mod parse_tests;

struct StmtCollector {
    stmts: Vec<AsmStmt>
}

impl StmtCollector {
    pub fn new() -> StmtCollector {
        StmtCollector {
            stmts: vec![]
        }
    }

    pub fn statements(&self) -> &Vec<AsmStmt> {
        &self.stmts
    }
}

impl SectionSink for StmtCollector {
    fn push_section(&mut self, _name: &str, stmts: Vec<AsmStmt>) {
        let mut stmts = stmts;
        self.stmts.append(&mut stmts);
    }
}