use super::{AsmLexer, AsmToken};

#[test]
fn asm_lexer_hex_literals() {
    let mut lex = AsmLexer::new("$32 $fff0 $deadbeef 0x0a3 0xF3");
    for token in lex.lexer() {
        assert_eq!(token, AsmToken::HexInteger);
    }
}

#[test]
fn asm_lexer_dec_literals() {
    let mut lex = AsmLexer::new("284 290 91");
    for token in lex.lexer() {
        assert_eq!(token, super::AsmToken::DecInteger);
    }
}
