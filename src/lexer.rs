use logos::Logos;

pub mod asm_tokens;
pub use asm_tokens::AsmToken;

mod asm_lexer_tests;

struct AsmLexer<'a> {
    lexer: logos::Lexer<'a, AsmToken>,
    last_token: AsmToken,
}

impl<'a> AsmLexer<'a> {
    fn new(source: &'a str) -> AsmLexer {
        AsmLexer {
            lexer: AsmToken::lexer(source),
            last_token: AsmToken::Error,
        }
    }

    fn next_token(&mut self) -> AsmToken {
        let token = match self.lexer.next() {
            Some(token) => token,
            None => AsmToken::Error,
        };
        self.last_token = token.clone();
        token
    }
}
