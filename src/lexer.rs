use logos::Logos;

pub mod asm_tokens;
pub use asm_tokens::AsmToken;

mod asm_lexer_tests;

pub struct AsmLexer<'a> {
    lexer: logos::Lexer<'a, AsmToken>,
    last_token: AsmToken,
}

impl<'a> AsmLexer<'a> {
    pub fn new(source: &'a str) -> AsmLexer {
        AsmLexer {
            lexer: AsmToken::lexer(source),
            last_token: AsmToken::Error,
        }
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
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
        self.last_token = token.clone();
        token
    }
}
