use crate::asm::{AsmParser, model::{AddrMode, AsmStmt, IndexMode, MemRef}, parser::tests::StmtCollector};

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
    let mut stmts = StmtCollector::new();
    parser.parse(&mut stmts);

    assert_eq!(parser.errors().len(), 0);
    assert_eq!(
        *stmts.statements(),
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
