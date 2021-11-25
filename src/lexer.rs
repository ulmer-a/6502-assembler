use logos::Logos;

pub mod asm_tokens;
pub use asm_tokens::AsmToken;

mod asm_lexer_tests;

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
            line: 1
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

    pub fn expect_one_of(&mut self, tokens: Vec<AsmToken>) {
        let token = self.lexer.next();
        if let Some(token) = token {
            if tokens.contains(&token) {
                return;
            }
        }
        panic!("unexpected token");
    }

    pub fn next_token(&mut self) -> AsmToken {
        let token = match self.lexer.next() {
            Some(token) => token,
            None => AsmToken::Error,
        };
        if self.current_token == AsmToken::Newline {
            self.line += 1;
        }
        self.current_token = token.clone();
        token
    }
}
