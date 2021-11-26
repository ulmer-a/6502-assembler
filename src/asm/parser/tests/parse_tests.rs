use crate::asm::{AsmParser, model::{AddrMode, AsmStmt, MemoryReference}};

#[test]
fn simple_labels() {
    let mut parser = AsmParser::new(&r#"
            brk
        my_label:
            no_label
        tw: lda variable
    "#);
    parser.parse();

    assert_eq!(parser.errors().len(), 0);
    assert_eq!(
        *parser.statements(),
        vec![
            AsmStmt::new_instr("brk".into(), AddrMode::Implied),
            AsmStmt::new_label("my_label".into()),
            AsmStmt::new_instr("no_label".into(), AddrMode::Implied),
            AsmStmt::new_label("tw".into()),
            AsmStmt::new_instr("lda".into(), AddrMode::Direct(MemoryReference::Variable("variable".into()))),
        ]
    );
}