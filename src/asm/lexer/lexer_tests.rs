use super::{AsmLexer, AsmToken};

#[test]
fn asm_lexer_hex_literals() {
    let mut lex = AsmLexer::new("$32 $fff0 $deadbeef 0x0a3 0xF3");

    let values: Vec<u64> = vec![ 0x32, 0xfff0, 0xdeadbeef, 0xa3, 0xf3 ];
    for value in values.iter() {
        assert_eq!(AsmToken::HexInteger, lex.next_token());
        assert_eq!(*value, lex.numeric_value().unwrap());
    }
}

#[test]
fn asm_lexer_dec_literals() {
    let mut lex = AsmLexer::new("284 290 91");

    let values: Vec<u64> = vec![ 284, 290, 91 ];
    for value in values.iter() {
        assert_eq!(AsmToken::DecInteger, lex.next_token());
        assert_eq!(*value, lex.numeric_value().unwrap());
    }
}
