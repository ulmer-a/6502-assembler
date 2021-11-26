#[cfg(test)]
mod instruction_parse_tests {
    use crate::{asm::model::*, AsmParser};

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
        parser.parse();

        assert_eq!(parser.errors().len(), 0);
        assert_eq!(
            *parser.statements(),
            vec![
                AsmStmt::AsmInstruction(Instruction::new("brk".into(), AddrMode::Implied)),
                AsmStmt::AsmInstruction(Instruction::new("inc".into(), AddrMode::Implied)),
                AsmStmt::AsmInstruction(Instruction::new("inx".into(), AddrMode::Implied)),
                AsmStmt::AsmInstruction(Instruction::new("dec".into(), AddrMode::Implied)),
                AsmStmt::AsmInstruction(Instruction::new("rts".into(), AddrMode::Implied)),
                AsmStmt::AsmInstruction(Instruction::new("lda".into(), AddrMode::Immediate(32))),
                AsmStmt::AsmInstruction(Instruction::new("cmp".into(), AddrMode::Immediate(0xf0))),
                AsmStmt::AsmInstruction(Instruction::new("rti".into(), AddrMode::Implied)),
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
        parser.parse();

        assert_eq!(parser.errors().len(), 0);
        assert_eq!(
            *parser.statements(),
            vec![
                AsmStmt::AsmInstruction(Instruction::new(
                    "jsr".into(),
                    AddrMode::Direct(MemoryReference::Variable("my_function".into()))
                )),
                AsmStmt::AsmInstruction(Instruction::new(
                    "lda".into(),
                    AddrMode::Direct(MemoryReference::Zeropage(0x32))
                )),
                AsmStmt::AsmInstruction(Instruction::new(
                    "stz".into(),
                    AddrMode::Direct(MemoryReference::Zeropage(0xff))
                )),
                AsmStmt::AsmInstruction(Instruction::new(
                    "ldx".into(),
                    AddrMode::Direct(MemoryReference::Zeropage(218))
                )),
                AsmStmt::AsmInstruction(Instruction::new(
                    "stx".into(),
                    AddrMode::Direct(MemoryReference::Absolute(0x8000))
                )),
            ]
        );
    }
}
