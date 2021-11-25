use super::asm_model::{AddrMode, Instruction, MemoryReference};
use super::lexer::{AsmLexer, AsmToken};

enum IndexMode {
    NoIndex,
    IndexedX,
    IndexedY,
}

pub struct AsmParser<'a> {
    lexer: AsmLexer<'a>,
    instructions: Vec<Instruction>,
}

impl<'a> AsmParser<'a> {
    pub fn new(source: &str) -> AsmParser {
        AsmParser {
            lexer: AsmLexer::new(source),
            instructions: vec![],
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.lexer.next_token() {
                AsmToken::Identifier => self.parse_instruction(),
                AsmToken::Error => return,
                _ => {
                    panic!("unexpected token");
                }
            }
        }
    }

    fn parse_instruction(&mut self) {
        let mnemonic: String = self.lexer.slice().into();
        let addr_mode = self.parse_addr_mode().unwrap();
        self.instructions
            .push(Instruction::new(mnemonic, addr_mode));
    }

    fn parse_addr_mode(&mut self) -> Option<AddrMode> {
        let addr_mode = match self.lexer.next_token() {
            AsmToken::Error | AsmToken::Semicolon | AsmToken::Newline => {
                return Some(AddrMode::Implied)
            }
            AsmToken::ImmediateModifier => {
                let token = self.lexer.next_token();
                let imm = self.parse_integer_literal(token)?;
                if imm >= 256 {
                    println!("error: immediate: number too large");
                    None
                } else {
                    Some(AddrMode::Immediate(imm as u8))
                }
            }
            token => self.parse_mem_addr_mode(token),
        };

        // fast forward to next token
        let end_tokens = vec![AsmToken::Semicolon, AsmToken::Newline];
        while !end_tokens.contains(&self.lexer.last_token()) {
            self.lexer.next_token();
        }

        addr_mode
    }

    fn parse_mem_addr_mode(&mut self, token: AsmToken) -> Option<AddrMode> {
        let mem_ref = self.parse_mem_ref(token)?;
        Some(AddrMode::Direct(mem_ref))
    }

    fn parse_mem_ref(&mut self, token: AsmToken) -> Option<MemoryReference> {
        match token {
            AsmToken::DecInteger | AsmToken::HexInteger => {
                let addr = self.parse_integer_literal(token)?;
                if addr < 0x100 {
                    Some(MemoryReference::Zeropage(addr as u8))
                } else if addr < 0x10000 {
                    Some(MemoryReference::Absolute(addr as u16))
                } else {
                    // Number too big
                    None
                }
            }
            AsmToken::Identifier => {
                Some(MemoryReference::Variable(String::from(self.lexer.slice())))
            }
            _ => None, // unexpected token
        }
    }

    fn parse_index_mode(&self) -> Option<IndexMode> {
        let id_text = self.lexer.slice().to_lowercase();
        match id_text.as_ref() {
            "x" => Some(IndexMode::IndexedX),
            "y" => Some(IndexMode::IndexedY),
            _ => {
                // error
                None
            }
        }
    }

    fn parse_integer_literal(&mut self, token: AsmToken) -> Option<u64> {
        let mut number_str = self.lexer.slice();
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
            _ => None,
        }
    }
}
