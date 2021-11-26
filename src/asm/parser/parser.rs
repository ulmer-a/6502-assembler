use super::{
    AsmLexer, AsmToken,
    super::model::*,
};

pub enum ErrorType {
    UnexpectedToken(AsmToken),
    ImmediateTooLarge,
    InvalidIndexRegister,
    AddressTooLarge,
}

pub struct CompileError {
    error_type: ErrorType,
    line: u32,
}

pub struct AsmParser<'a> {
    lexer: AsmLexer<'a>,
    instructions: Vec<Instruction>,
    errors: Vec<CompileError>,
}

impl<'a> AsmParser<'a> {
    pub fn new(source: &str) -> AsmParser {
        AsmParser {
            lexer: AsmLexer::new(source),
            instructions: vec![],
            errors: vec![],
        }
    }

    fn error(&mut self, error_type: ErrorType) {
        self.errors.push(CompileError {
            error_type,
            line: self.lexer.line()
        });
    }

    pub fn parse(&mut self) {
        loop {
            let token = self.lexer.next_token();
            match token {
                AsmToken::Identifier => self.parse_instruction(),
                AsmToken::Error => return,
                _ => {
                    self.error(ErrorType::UnexpectedToken(token));
                }
            }
        }
    }

    fn parse_instruction(&mut self) {
        let mnemonic: String = self.lexer.slice().into();
        if let Some(addr_mode) = self.parse_addr_mode() {
            self.instructions
                .push(Instruction::new(mnemonic, addr_mode));
        }
    }

    fn parse_addr_mode(&mut self) -> Option<AddrMode> {
        match self.lexer.next_token() {
            AsmToken::Error | AsmToken::Semicolon | AsmToken::Newline => {
                Some(AddrMode::Implied)
            },
            AsmToken::ImmediateModifier => self.parse_immediate(),
            _ => self.parse_mem_addr_mode(),
        }
    }

    fn parse_immediate(&mut self) -> Option<AddrMode> {
        let value = self.parse_integer_literal()?;
        if value >= 256 {
            self.error(ErrorType::ImmediateTooLarge);
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
        if self.lexer.next_token() == AsmToken::Colon {
            let id_token = self.lexer.next_token();
            if let AsmToken::Identifier = id_token {
                self.parse_index_mode(mem_ref)
            } else {
                self.error(ErrorType::UnexpectedToken(id_token));
                None
            }
        } else {
            Some(AddrMode::Direct(mem_ref))
        }
    }

    fn parse_mem_ref(&mut self) -> Option<MemoryReference> {
        let token = self.lexer.current_token();
        match token {
            AsmToken::DecInteger | AsmToken::HexInteger => {
                let addr = self.parse_integer_literal()?;
                if addr < 0x100 {
                    Some(MemoryReference::Zeropage(addr as u8))
                } else if addr < 0x10000 {
                    Some(MemoryReference::Absolute(addr as u16))
                } else {
                    self.error(ErrorType::AddressTooLarge);
                    None
                }
            }
            AsmToken::Identifier => {
                Some(MemoryReference::Variable(String::from(self.lexer.slice())))
            }
            _ => {
                self.error(ErrorType::UnexpectedToken(token));
                None
            }
        }
    }

    fn parse_index_mode(&mut self, mem_ref: MemoryReference) -> Option<AddrMode> {
        let id_text = self.lexer.slice().to_lowercase();
        match id_text.as_ref() {
            "x" => Some(AddrMode::DirectIndexedX(mem_ref)),
            "y" => Some(AddrMode::DirectIndexedY(mem_ref)),
            _ => {
                self.error(ErrorType::InvalidIndexRegister);
                None
            }
        }
    }

    fn parse_integer_literal(&mut self) -> Option<u64> {
        let mut number_str = self.lexer.slice();
        let token = self.lexer.current_token();
        match token {
            AsmToken::HexInteger => {
                if number_str.chars().next().unwrap() == '$' {
                    number_str = &number_str[1..];
                } else {
                    number_str = &number_str[2..];
                }
                Some(u64::from_str_radix(number_str, 16).unwrap())
            }
            AsmToken::DecInteger => Some(u64::from_str_radix(&number_str, 10).unwrap()),
            _ => {
                self.error(ErrorType::UnexpectedToken(token));
                None
            }
        }
    }
}
