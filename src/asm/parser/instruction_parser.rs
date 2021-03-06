use super::{super::model::*, AsmParseError, AsmParser, AsmToken};
use crate::asm::model::Instruction;

impl<'a> AsmParser<'a> {
    pub fn parse_instruction(&mut self, mnemonic: String) {
        if let Some(addr_mode) = self.parse_addr_mode() {
            self.statements
                .push(AsmStmt::AsmInstruction(Instruction::new(
                    mnemonic, addr_mode,
                )));
        }
    }

    fn parse_addr_mode(&mut self) -> Option<AddrMode> {
        self.parse_until(vec![AsmToken::Newline, AsmToken::Semicolon], |p| {
            match p.lexer.current_token() {
                AsmToken::End | AsmToken::Semicolon | AsmToken::Newline => Some(AddrMode::Implied),
                AsmToken::ImmediateModifier => p.parse_immediate(),
                _ => p.parse_mem_addr_mode(),
            }
        })
    }

    fn parse_immediate(&mut self) -> Option<AddrMode> {
        self.lexer.next_token();
        let value = self.lexer.numeric_value()?;
        if value >= 256 {
            self.error(AsmParseError::ImmediateTooLarge);
            None
        } else {
            Some(AddrMode::Immediate(value as u8))
        }
    }

    fn parse_mem_addr_mode(&mut self) -> Option<AddrMode> {
        self.parse_indexed_mem_ref()
    }

    fn parse_indexed_mem_ref(&mut self) -> Option<AddrMode> {
        let mem_ref = self.parse_mem_ref()?;
        if self.lexer.next_token() == AsmToken::Comma {
            let id_token = self.lexer.next_token();
            if let AsmToken::Identifier = id_token {
                self.parse_index_mode(mem_ref)
            } else {
                self.error(AsmParseError::UnexpectedToken(id_token));
                None
            }
        } else {
            Some(AddrMode::Memory(IndexMode::None, mem_ref))
        }
    }

    pub fn parse_mem_ref(&mut self) -> Option<MemRef> {
        let token = self.lexer.current_token();
        match token {
            AsmToken::DecInteger | AsmToken::HexInteger => {
                let addr = self.parse_integer_literal()?;
                if addr < 0x10000 {
                    Some(MemRef::Addr(addr as u16))
                } else {
                    self.error(AsmParseError::AddressTooLarge);
                    None
                }
            }
            AsmToken::Identifier => Some(MemRef::Variable(String::from(self.lexer.slice()))),
            _ => {
                self.error(AsmParseError::UnexpectedToken(token));
                None
            }
        }
    }

    fn parse_index_mode(&mut self, mem_ref: MemRef) -> Option<AddrMode> {
        let id_text = self.lexer.slice().to_lowercase();
        match id_text.as_ref() {
            "x" => Some(AddrMode::Memory(IndexMode::IndexedX, mem_ref)),
            "y" => Some(AddrMode::Memory(IndexMode::IndexedY, mem_ref)),
            _ => {
                self.error(AsmParseError::InvalidIndexRegister(id_text));
                None
            }
        }
    }

    fn parse_integer_literal(&mut self) -> Option<u64> {
        self.lexer.numeric_value()
    }
}
