use crate::asm::{
    model::{AddrMode, AsmStmt, IndexMode, MemRef},
    AsmParser,
};

#[test]
fn simple_labels() {
    let mut parser = AsmParser::new(
        &r#"
            brk
            driver_addr = $34;
        my_label:
            no_label
        tw: lda variable
    "#,
    );
    parser.parse();

    assert_eq!(parser.errors().len(), 0);
    assert_eq!(
        *parser.statements(),
        vec![
            AsmStmt::new_instr("brk".into(), AddrMode::Implied),
            AsmStmt::new_const_label("driver_addr".into(), 0x34),
            AsmStmt::new_label("my_label".into()),
            AsmStmt::new_instr("no_label".into(), AddrMode::Implied),
            AsmStmt::new_label("tw".into()),
            AsmStmt::new_instr(
                "lda".into(),
                AddrMode::Memory(IndexMode::None, MemRef::Variable("variable".into()))
            ),
        ]
    );
}
