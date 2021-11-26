#[cfg(test)]
mod instruction_parse_tests {
    use crate::AsmParser;

    #[test]
    fn parse_implied_and_immediate() {
        let mut parser = AsmParser::new(
            &r#"
            brk
            inc ; inx
            dec;
            rts
            lda #32;
            cmp #$0x100 ; rti
        "#,
        );
        parser.parse();

        assert_eq!(parser.instructions().len(), 8);
        assert_eq!(parser.errors().len(), 0);
    }
}
