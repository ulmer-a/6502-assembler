mod errors;
mod instruction_parse_tests;
mod instruction_parser;

use super::lexer::{AsmLexer, AsmToken};
use crate::{asm::model::Instruction, errors::CompileError};
use errors::AsmParseError;

pub struct AsmParser<'a> {
    lexer: AsmLexer<'a>,
    instructions: Vec<Instruction>,
    errors: Vec<CompileError<AsmParseError>>,
}

impl<'a> AsmParser<'a> {
    pub fn new(source: &str) -> AsmParser {
        AsmParser {
            lexer: AsmLexer::new(source),
            instructions: vec![],
            errors: vec![],
        }
    }

    #[cfg(test)]
    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    #[cfg(test)]
    pub fn errors(&self) -> &Vec<CompileError<AsmParseError>> {
        &self.errors
    }

    pub fn dump_errors(&self) {
        for error in self.errors.iter() {
            error.print();
        }
    }

    fn error(&mut self, error_type: AsmParseError) {
        self.errors
            .push(CompileError::new(error_type, self.lexer.line()));
    }

    fn insert_label(&mut self, name: String) {
        println!("generate label: {}", name);
    }

    pub fn parse(&mut self) {
        loop {
            match self.lexer.next_token() {
                AsmToken::Identifier => {
                    // lines or expressions starting with an identifier could be
                    // either instructions, labels or label assignments, so some
                    // lookahead has to be performed.
                    let identifier: String = self.lexer.slice().into();
                    match self.lexer.next_token() {
                        AsmToken::Colon => self.insert_label(identifier),
                        _ => self.parse_instruction(identifier),
                    }
                }
                AsmToken::End => return,
                AsmToken::Newline | AsmToken::Semicolon => {}
                token => {
                    self.error(AsmParseError::UnexpectedToken(token));
                }
            }
        }
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
}
