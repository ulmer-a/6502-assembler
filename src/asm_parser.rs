use super::asm_model::AddrMode;
use super::lexer::{AsmLexer, AsmToken};

pub struct AsmParser<'a> {
    lexer: AsmLexer<'a>,
}

impl<'a> AsmParser<'a> {
    pub fn new(source: &str) -> AsmParser {
        AsmParser {
            lexer: AsmLexer::new(source),
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.lexer.next_token() {
                AsmToken::Mnemonic => self.parse_instruction(),
                AsmToken::Error => return,
                _ => {
                    panic!("unexpected token");
                }
            }
        }
    }

    pub fn parse_instruction(&mut self) {
        let _mnemonic: String = self.lexer.slice().into();
        let _addr_mode = self.parse_addr_mode().unwrap();
        let _addr_mode = match _addr_mode {
            AddrMode::Implied => String::from("implied"),
            AddrMode::Immediate(i) => format!("imm #{}", i),
        };
        println!("mnemonic={}, addr_mode={}", _mnemonic, _addr_mode);
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
