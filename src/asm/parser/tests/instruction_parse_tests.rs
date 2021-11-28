use crate::{
    asm::{model::*, parser::tests::StmtCollector},
    AsmParser,
};

#[test]
fn parse_implied_and_immediate() {
    let mut parser = AsmParser::new(
        &r#"
        brk
        inc ; inx
        dec;
        rts
        lda #32;
        cmp #0xf0 ; rti
    "#,
    );
    let mut stmts = StmtCollector::new();
    parser.parse(&mut stmts);

    assert_eq!(parser.errors().len(), 0);
    assert_eq!(
        *stmts.statements(),
        vec![
            AsmStmt::new_instr("brk".into(), AddrMode::Implied),
            AsmStmt::new_instr("inc".into(), AddrMode::Implied),
            AsmStmt::new_instr("inx".into(), AddrMode::Implied),
            AsmStmt::new_instr("dec".into(), AddrMode::Implied),
            AsmStmt::new_instr("rts".into(), AddrMode::Implied),
            AsmStmt::new_instr("lda".into(), AddrMode::Immediate(32)),
            AsmStmt::new_instr("cmp".into(), AddrMode::Immediate(0xf0)),
            AsmStmt::new_instr("rti".into(), AddrMode::Implied),
        ]
    );
}

#[test]
fn parse_direct_mem_refs() {
    let mut parser = AsmParser::new(
        &r#"
        jsr my_function
        lda $32
        stz 0xff
        ldx 218
        stx 0x8000
    "#,
    );
    let mut stmts = StmtCollector::new();
    parser.parse(&mut stmts);

    assert_eq!(parser.errors().len(), 0);
    assert_eq!(
        *stmts.statements(),
        vec![
            AsmStmt::new_instr(
                "jsr".into(),
                AddrMode::Memory(IndexMode::None, MemRef::Variable("my_function".into()))
            ),
            AsmStmt::new_instr(
                "lda".into(),
                AddrMode::Memory(IndexMode::None, MemRef::Addr(0x32))
            ),
            AsmStmt::new_instr(
                "stz".into(),
                AddrMode::Memory(IndexMode::None, MemRef::Addr(0xff))
            ),
            AsmStmt::new_instr(
                "ldx".into(),
                AddrMode::Memory(IndexMode::None, MemRef::Addr(218))
            ),
            AsmStmt::new_instr(
                "stx".into(),
                AddrMode::Memory(IndexMode::None, MemRef::Addr(0x8000))
            ),
        ]
    );
}
