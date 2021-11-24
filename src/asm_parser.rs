use super::asm_model::{AddrMode, Instruction};
use super::lexer::{AsmLexer, AsmToken};

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

    pub fn parse_instruction(&mut self) {
        let mnemonic: String = self.lexer.slice().into();
        let addr_mode = self.parse_addr_mode().unwrap();
        self.instructions
            .push(Instruction::new(mnemonic, addr_mode));
    }

    pub fn parse_addr_mode(&mut self) -> Option<AddrMode> {
        let addr_mode = match self.lexer.next_token() {
            AsmToken::Error | AsmToken::Semicolon | AsmToken::Newline => {
                return Some(AddrMode::Implied)
            }
            AsmToken::ImmediateModifier => {
                let imm = self.parse_integer_literal()?;
                if imm >= 256 {
                    println!("error: immediate: number too large");
                    None
                } else {
                    Some(AddrMode::Immediate(imm as u8))
                }
            }
            _ => {
                panic!("unexpected token");
            }
        };
        self.lexer
            .expect_one_of(vec![AsmToken::Semicolon, AsmToken::Newline]);
        addr_mode
    }

    pub fn parse_integer_literal(&mut self) -> Option<u64> {
        let token = self.lexer.next_token();
        let number_str = self.lexer.slice();
        match token {
            AsmToken::HexInteger => Some(u64::from_str_radix(&number_str[2..], 16).unwrap()),
            AsmToken::DecInteger => Some(u64::from_str_radix(&number_str, 10).unwrap()),
            _ => {
                println!("unexpected token: expected integer literal");
                None
            }
        }
    }
}
