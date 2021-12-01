mod errors;
mod instruction_parser;

#[cfg(test)]
mod tests;

use super::{
    lexer::{AsmLexer, AsmToken},
    model::AsmStmt,
};
use crate::{asm::model::DataPlacement, errors::CompileError};
use errors::AsmParseError;

pub struct AsmParser<'a> {
    lexer: AsmLexer<'a>,
    errors: Vec<CompileError<AsmParseError>>,
    current_section_name: String,
    statements: Vec<AsmStmt>,
}

pub trait SectionSink {
    fn push_section(&mut self, name: &str, stmts: Vec<AsmStmt>);
}

impl<'a> AsmParser<'a> {
    pub fn new(source: &str) -> AsmParser {
        AsmParser {
            lexer: AsmLexer::new(source),
            errors: vec![],
            current_section_name: "text".into(),
            statements: vec![],
        }
    }

    #[cfg(test)]
    pub fn errors(&self) -> &Vec<CompileError<AsmParseError>> {
        &self.errors
    }

    pub fn dump_errors(&self) -> usize {
        let mut error_count = 0;
        for error in self.errors.iter() {
            error.print();
            error_count += 1;
        }
        error_count
    }

    fn error(&mut self, error_type: AsmParseError) {
        self.errors
            .push(CompileError::new(error_type, self.lexer.line()));
    }

    fn insert_label(&mut self, name: String, addr: Option<u16>) {
        if let Some(addr) = addr {
            self.statements.push(AsmStmt::ConstLabel(name, addr));
        } else {
            self.statements.push(AsmStmt::Label(name));
        }
    }

    pub fn parse<T: SectionSink>(&mut self, sink: &mut T) {
        loop {
            match self.lexer.next_token() {
                AsmToken::Identifier => {
                    // lines or expressions starting with an identifier could be
                    // either instructions, labels or label assignments, so some
                    // lookahead has to be performed.
                    let identifier: String = self.lexer.slice().into();
                    match self.lexer.next_token() {
                        AsmToken::Colon => self.insert_label(identifier, None),
                        AsmToken::AssignmentOperator => self.parse_const_addr(identifier),
                        _ => self.parse_instruction(identifier),
                    }
                }
                AsmToken::SectionKeyword => {
                    let token = self.lexer.next_token();
                    if token == AsmToken::Identifier {
                        sink.push_section(
                            &self.current_section_name,
                            std::mem::replace(&mut self.statements, vec![]),
                        );
                        self.current_section_name = self.lexer.slice().into();
                    } else {
                        self.error(AsmParseError::UnexpectedToken(token))
                    }
                }
                AsmToken::StrKeyword => {
                    let token = self.lexer.next_token();
                    if token == AsmToken::StringLiteral {
                        let str_value = self.lexer.slice();
                        self.statements.push(AsmStmt::Data(DataPlacement::Str(
                            (&str_value[1..str_value.len() - 1]).into(),
                        )));
                    } else {
                        self.error(AsmParseError::UnexpectedToken(token))
                    }
                },
                AsmToken::WordKeyword => {
                    self.lexer.next_token();
                    if let Some(mem_ref) = self.parse_mem_ref() {
                        self.statements.push(AsmStmt::Data(DataPlacement::Word(
                            mem_ref
                        )));
                    }
                },
                AsmToken::End => break,
                AsmToken::Newline | AsmToken::Semicolon => {}
                token => {
                    self.error(AsmParseError::UnexpectedToken(token));
                }
            }
        }
        sink.push_section(
            &self.current_section_name,
            std::mem::replace(&mut self.statements, vec![]),
        );
    }

    fn parse_until<T, F: Fn(&mut Self) -> T>(&mut self, end_tokens: Vec<AsmToken>, func: F) -> T {
        let result = func(self);

        let until_condition = |t: &&AsmToken| !end_tokens.contains(t) && t != &&AsmToken::End;
        let mut unexpected_tokens: Vec<AsmToken> = vec![];
        while until_condition(&&self.lexer.current_token()) {
            unexpected_tokens.push(self.lexer.next_token());
        }

        let excess_tokens = unexpected_tokens.iter().filter(until_condition).count();
        if excess_tokens > 0 {
            self.error(AsmParseError::ExcessTokens(excess_tokens));
        }

        result
    }

    fn parse_const_addr(&mut self, name: String) {
        let token = self.lexer.next_token(); // skip assignment operator
        if let Some(addr) = self.lexer.numeric_value() {
            if addr <= 0xffff {
                self.insert_label(name, Some(addr as u16));
            } else {
                self.error(AsmParseError::AddressTooLarge);
            }
        } else {
            self.error(AsmParseError::UnexpectedToken(token));
        }
    }
}
