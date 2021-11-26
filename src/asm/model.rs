use std::{str::FromStr};
use strum::EnumString;

#[derive(EnumString)]
pub enum Mnemonic {
    ADC, // add with carry
    AND, // and with A register
    ASL, // arithmetic shift left
    BBR, // branch if bit clear
    BBS, // branch if bit set
    BCC, // branch if carry clear
    BCS, // branch if carry set
    BEQ, // branch if equal, branch if zero
    BIT, // and with A register, no writeback
    BMI, // branch if negative
    BNE, // branch if not equal, branch if not zero
    BPL, // branch if positive
    BRA, // branch always
    BRK, // break instruction
    BVC, // branch if overflow clear
    BVS, // branch if overflow set
    CLC, // clear carry
    CLD, // clear decimal flag
    CLI, // enable interrupts
    CLV, // clear overflow flag
    CMP, // compare A register
    CPX, // compare X register
    CPY, // compare Y register
    DEC, // decrement A register or memory
    DEX, // decrement X register
    DEY, // decrement Y register
    EOR, // xor with A register
    INC, // increment A register or memory
    INX, // increment X register
    INY, // increment Y register
    JMP, // jump
    JSR, // jump to subroutine (call)
    LDA, // load A register
    LDX, // load X register
    LDY, // load Y register
    LSR, // logical shift right
    NOP, // no operation
    ORA, // or with A register
    PHA, // push A register to stack
    PHP, // push status register to stack
    PHX, // push X register to stack
    PHY, // push Y register to stack
    PLA, // pop A register from stack
    PLP, // pull status register from stack
    PLX, // pull X register from stack
    PLY, // pull Y register from stack
    RMB, // clear bit in memory
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SBC, // subtract with carry
    SEC, // set carry flag
    SED, // set decimal flag
    SEI, // disable interrupts
    SMB, // set memory bit
    STA, // store A register in memory
    STP, // stop CPU clock (halt)
    STX, // store X register in memory
    STY, // store Y register in memory
    STZ, // store zero in memory
    TAX, // transfer A -> X
    TAY, // transfer A -> Y
    TRB, // test and clear memory bit
    TSB, // test and set memory bit
    TSX, // transfer SP -> X
    TXA, // transfer X -> A
    TXS, // transfer X -> SP
    TYA, // transfer Y -> A
    WAI, // wait for interrupt
}

pub enum AddrMode {
    Implied,
    Immediate(u8),
    Direct(MemoryReference),
    DirectIndexedX(MemoryReference),
    DirectIndexedY(MemoryReference),
}

#[derive(Clone)]
pub enum MemoryReference {
    Variable(String),
    Zeropage(u8),
    Absolute(u16),
}

pub struct Instruction {
    mnemonic: Mnemonic,
    addr_mode: AddrMode,
}

impl Instruction {
    pub fn new(mnemonic: String, addr_mode: AddrMode) -> Instruction {
        Instruction {
            mnemonic: Mnemonic::from_str(&mnemonic.to_uppercase()).unwrap(),
            addr_mode: addr_mode,
        }
    }
}
