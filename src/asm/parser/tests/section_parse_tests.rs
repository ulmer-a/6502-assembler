use crate::asm::{
    model::{AddrMode, AsmStmt},
    parser::tests::StmtCollector,
    AsmParser,
};

#[test]
fn simple_sections() {
    let mut parser = AsmParser::new(
        &r#"
            brk
            clc
            section other_section
            cli
            section text
            sei
        "#,
    );
    let mut stmts = StmtCollector::new();
    parser.parse(&mut stmts);

    assert_eq!(parser.errors().len(), 0);
    assert_eq!(
        *stmts.section_statements("text"),
        vec![
            AsmStmt::new_instr("brk".into(), AddrMode::Implied),
            AsmStmt::new_instr("clc".into(), AddrMode::Implied),
            AsmStmt::new_instr("sei".into(), AddrMode::Implied),
        ]
    );
    assert_eq!(
        *stmts.section_statements("other_section"),
        vec![AsmStmt::new_instr("cli".into(), AddrMode::Implied),]
    );
}
