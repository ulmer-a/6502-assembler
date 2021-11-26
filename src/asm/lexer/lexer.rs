use super::AsmToken;
use logos::Logos;

pub struct AsmLexer<'a> {
    lexer: logos::Lexer<'a, AsmToken>,
    current_token: AsmToken,
    line: u32,
}

impl<'a> AsmLexer<'a> {
    pub fn new(source: &'a str) -> AsmLexer {
        AsmLexer {
            lexer: AsmToken::lexer(source),
            current_token: AsmToken::Error,
            line: 1,
        }
    }

    pub fn numeric_value(&self) -> Option<u64> {
        let mut number_str = self.lexer.slice();
        match self.current_token {
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

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn current_token(&self) -> AsmToken {
        self.current_token.clone()
    }

    pub fn next_token(&mut self) -> AsmToken {
        let token = match self.lexer.next() {
            Some(token) => token,
            None => AsmToken::End,
        };
        if self.current_token == AsmToken::Newline {
            self.line += 1;
        }
        self.current_token = token.clone();
        token
    }
}
