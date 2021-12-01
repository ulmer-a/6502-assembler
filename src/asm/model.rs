use super::codegen::get_opcode;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, PartialEq)]
pub enum AsmStmt {
    AsmInstruction(Instruction),
    Data(DataPlacement),
    Label(String),
    ConstLabel(String, u16),
}

impl AsmStmt {
    #[cfg(test)]
    pub fn new_instr(mnemonic: String, addr_mode: AddrMode) -> AsmStmt {
        AsmStmt::AsmInstruction(Instruction::new(mnemonic, addr_mode))
    }

    #[cfg(test)]
    pub fn new_label(name: String) -> AsmStmt {
        AsmStmt::Label(name)
    }

    #[cfg(test)]
    pub fn new_const_label(name: String, addr: u16) -> AsmStmt {
        AsmStmt::ConstLabel(name, addr)
    }
}

#[derive(Debug, PartialEq)]
pub enum DataPlacement {
    Str(String),
    Word(MemRef),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    mnemonic: Mnemonic,
    addr_mode: AddrMode,
}

#[derive(EnumString, Debug, PartialEq, Copy, Clone)]
pub enum Mnemonic {
    BRK,
    ORA,
    NOP,
    TSB,
    ASL,
    RMB0,
    PHP,
    BBR0,
    BPL,
    TRB,
    RMB1,
    CLC,
    INC,
    BBR1,
    JSR,
    AND,
    BIT,
    ROL,
    RMB2,
    PLP,
    BBR2,
    BMI,
    RMB3,
    SEC,
    DEC,
    BBR3,
    RTI,
    EOR,
    LSR,
    RMB4,
    PHA,
    JMP,
    BBR4,
    BVC,
    RMB5,
    CLI,
    PHY,
    BBR5,
    RTS,
    ADC,
    STZ,
    ROR,
    RMB6,
    PLA,
    BBR6,
    BVS,
    RMB7,
    SEI,
    PLY,
    BBR7,
    BRA,
    STA,
    STY,
    STX,
    SMB0,
    DEY,
    TXA,
    BBS0,
    BCC,
    SMB1,
    TYA,
    TXS,
    BBS1,
    LDY,
    LDA,
    LDX,
    SMB2,
    TAY,
    TAX,
    BBS2,
    BCS,
    SMB3,
    CLV,
    TSX,
    BBS3,
    CPY,
    CMP,
    SMB4,
    INY,
    DEX,
    WAI,
    BBS4,
    BNE,
    SMB5,
    CLD,
    PHX,
    STP,
    BBS5,
    CPX,
    SBC,
    SMB6,
    INX,
    BBS6,
    BEQ,
    SMB7,
    SED,
    PLX,
    BBS7,

    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndexMode {
    IndexedX,
    IndexedY,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AddrMode {
    Implied,
    Immediate(u8),
    Memory(IndexMode, MemRef),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MemRef {
    Variable(String),
    Addr(u16),
}

impl Instruction {
    pub fn new(mnemonic: String, addr_mode: AddrMode) -> Instruction {
        Instruction {
            mnemonic: match Mnemonic::from_str(&mnemonic.to_uppercase()) {
                Ok(m) => m,
                Err(_) => Mnemonic::Invalid,
            },
            addr_mode,
        }
    }

    pub fn addr_mode(&self) -> AddrMode {
        self.addr_mode.clone()
    }

    pub fn mnemonic_index(&self) -> usize {
        self.mnemonic as usize
    }

    pub fn has_rel_addressing(&self) -> bool {
        get_opcode(self.mnemonic_index(), 13).is_some()
    }
}
