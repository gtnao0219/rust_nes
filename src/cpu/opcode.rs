use crate::{log, Byte, Cycle};

#[derive(Debug, Eq, PartialEq)]
pub enum Addressing {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpcodeBaseName {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    ADC,
    AND,
    ASL,
    BIT,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    LSR,
    ORA,
    ROL,
    ROR,
    SBC,

    PHA,
    PHP,
    PLA,
    PLP,

    JMP,
    JSR,
    RTS,
    RTI,

    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,

    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,

    BRK,
    NOP,

    // unofficial opcodes
    NOPD,
    NOPI,
    LAX,
    SAX,
    DCP,
    ISB,
    SLO,
    RLA,
    SRE,
    RRA,
}

#[derive(Debug)]
pub struct Opcode {
    pub base_name: OpcodeBaseName,
    pub addressing: Addressing,
    pub cycle: Cycle,
}

pub fn get_opcode(byte: Byte) -> Opcode {
    match byte {
        0xA9 => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xA5 => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xAD => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xB5 => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0xBD => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0xB9 => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0xA1 => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0xB1 => Opcode {
            base_name: OpcodeBaseName::LDA,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0xA2 => Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xA6 => Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xAE => Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xB6 => Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::ZeroPageY,
            cycle: 4,
        },
        0xBE => Opcode {
            base_name: OpcodeBaseName::LDX,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0xA0 => Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xA4 => Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xAC => Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xB4 => Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0xBC => Opcode {
            base_name: OpcodeBaseName::LDY,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0x85 => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x8D => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x95 => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0x9D => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0x99 => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0x81 => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0x91 => Opcode {
            base_name: OpcodeBaseName::STA,
            addressing: Addressing::IndirectY,
            cycle: 6,
        },
        0x86 => Opcode {
            base_name: OpcodeBaseName::STX,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x8E => Opcode {
            base_name: OpcodeBaseName::STX,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x96 => Opcode {
            base_name: OpcodeBaseName::STX,
            addressing: Addressing::ZeroPageY,
            cycle: 4,
        },
        0x84 => Opcode {
            base_name: OpcodeBaseName::STY,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x8C => Opcode {
            base_name: OpcodeBaseName::STY,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x94 => Opcode {
            base_name: OpcodeBaseName::STY,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0x8A => Opcode {
            base_name: OpcodeBaseName::TXA,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x98 => Opcode {
            base_name: OpcodeBaseName::TYA,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x9A => Opcode {
            base_name: OpcodeBaseName::TXS,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xA8 => Opcode {
            base_name: OpcodeBaseName::TAY,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xAA => Opcode {
            base_name: OpcodeBaseName::TAX,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xBA => Opcode {
            base_name: OpcodeBaseName::TSX,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x08 => Opcode {
            base_name: OpcodeBaseName::PHP,
            addressing: Addressing::Implied,
            cycle: 3,
        },
        0x28 => Opcode {
            base_name: OpcodeBaseName::PLP,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x48 => Opcode {
            base_name: OpcodeBaseName::PHA,
            addressing: Addressing::Implied,
            cycle: 3,
        },
        0x68 => Opcode {
            base_name: OpcodeBaseName::PLA,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x69 => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0x65 => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x6D => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x75 => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0x7D => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0x79 => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0x61 => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0x71 => Opcode {
            base_name: OpcodeBaseName::ADC,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0xE9 => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xE5 => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xED => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xF5 => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0xFD => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0xF9 => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0xE1 => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0xF1 => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0xE0 => Opcode {
            base_name: OpcodeBaseName::CPX,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xE4 => Opcode {
            base_name: OpcodeBaseName::CPX,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xEC => Opcode {
            base_name: OpcodeBaseName::CPX,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xC0 => Opcode {
            base_name: OpcodeBaseName::CPY,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xC4 => Opcode {
            base_name: OpcodeBaseName::CPY,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xCC => Opcode {
            base_name: OpcodeBaseName::CPY,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xC9 => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xC5 => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xCD => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xD5 => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0xDD => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0xD9 => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0xC1 => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0xD1 => Opcode {
            base_name: OpcodeBaseName::CMP,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0x29 => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0x25 => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x2D => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x35 => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0x3D => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0x39 => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0x21 => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0x31 => Opcode {
            base_name: OpcodeBaseName::AND,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0x49 => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0x45 => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x4D => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x55 => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0x5D => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0x59 => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0x41 => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0x51 => Opcode {
            base_name: OpcodeBaseName::EOR,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0x09 => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0x05 => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x0D => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x15 => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::ZeroPageX,
            cycle: 4,
        },
        0x1D => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::AbsoluteX,
            cycle: 4,
        },
        0x19 => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0x01 => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0x11 => Opcode {
            base_name: OpcodeBaseName::ORA,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0x24 => Opcode {
            base_name: OpcodeBaseName::BIT,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x2C => Opcode {
            base_name: OpcodeBaseName::BIT,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x0A => Opcode {
            base_name: OpcodeBaseName::ASL,
            addressing: Addressing::Accumulator,
            cycle: 2,
        },
        0x06 => Opcode {
            base_name: OpcodeBaseName::ASL,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x0E => Opcode {
            base_name: OpcodeBaseName::ASL,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x16 => Opcode {
            base_name: OpcodeBaseName::ASL,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x1E => Opcode {
            base_name: OpcodeBaseName::ASL,
            addressing: Addressing::AbsoluteX,
            cycle: 6,
        },
        0x4A => Opcode {
            base_name: OpcodeBaseName::LSR,
            addressing: Addressing::Accumulator,
            cycle: 2,
        },
        0x46 => Opcode {
            base_name: OpcodeBaseName::LSR,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x4E => Opcode {
            base_name: OpcodeBaseName::LSR,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x56 => Opcode {
            base_name: OpcodeBaseName::LSR,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x5E => Opcode {
            base_name: OpcodeBaseName::LSR,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0x2A => Opcode {
            base_name: OpcodeBaseName::ROL,
            addressing: Addressing::Accumulator,
            cycle: 2,
        },
        0x26 => Opcode {
            base_name: OpcodeBaseName::ROL,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x2E => Opcode {
            base_name: OpcodeBaseName::ROL,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x36 => Opcode {
            base_name: OpcodeBaseName::ROL,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x3E => Opcode {
            base_name: OpcodeBaseName::ROL,
            addressing: Addressing::AbsoluteX,
            cycle: 6,
        },
        0x6A => Opcode {
            base_name: OpcodeBaseName::ROR,
            addressing: Addressing::Accumulator,
            cycle: 2,
        },
        0x66 => Opcode {
            base_name: OpcodeBaseName::ROR,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x6E => Opcode {
            base_name: OpcodeBaseName::ROR,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x76 => Opcode {
            base_name: OpcodeBaseName::ROR,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x7E => Opcode {
            base_name: OpcodeBaseName::ROR,
            addressing: Addressing::AbsoluteX,
            cycle: 6,
        },
        0xE8 => Opcode {
            base_name: OpcodeBaseName::INX,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xC8 => Opcode {
            base_name: OpcodeBaseName::INY,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xE6 => Opcode {
            base_name: OpcodeBaseName::INC,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0xEE => Opcode {
            base_name: OpcodeBaseName::INC,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0xF6 => Opcode {
            base_name: OpcodeBaseName::INC,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0xFE => Opcode {
            base_name: OpcodeBaseName::INC,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0xCA => Opcode {
            base_name: OpcodeBaseName::DEX,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x88 => Opcode {
            base_name: OpcodeBaseName::DEY,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xC6 => Opcode {
            base_name: OpcodeBaseName::DEC,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0xCE => Opcode {
            base_name: OpcodeBaseName::DEC,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0xD6 => Opcode {
            base_name: OpcodeBaseName::DEC,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0xDE => Opcode {
            base_name: OpcodeBaseName::DEC,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0x18 => Opcode {
            base_name: OpcodeBaseName::CLC,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x58 => Opcode {
            base_name: OpcodeBaseName::CLI,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xB8 => Opcode {
            base_name: OpcodeBaseName::CLV,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x38 => Opcode {
            base_name: OpcodeBaseName::SEC,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x78 => Opcode {
            base_name: OpcodeBaseName::SEI,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xEA => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x00 => Opcode {
            base_name: OpcodeBaseName::BRK,
            addressing: Addressing::Implied,
            cycle: 7,
        },
        0x20 => Opcode {
            base_name: OpcodeBaseName::JSR,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x4C => Opcode {
            base_name: OpcodeBaseName::JMP,
            addressing: Addressing::Absolute,
            cycle: 3,
        },
        0x6C => Opcode {
            base_name: OpcodeBaseName::JMP,
            addressing: Addressing::Indirect,
            cycle: 5,
        },
        0x40 => Opcode {
            base_name: OpcodeBaseName::RTI,
            addressing: Addressing::Implied,
            cycle: 6,
        },
        0x60 => Opcode {
            base_name: OpcodeBaseName::RTS,
            addressing: Addressing::Implied,
            cycle: 6,
        },
        0x10 => Opcode {
            base_name: OpcodeBaseName::BPL,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0x30 => Opcode {
            base_name: OpcodeBaseName::BMI,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0x50 => Opcode {
            base_name: OpcodeBaseName::BVC,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0x70 => Opcode {
            base_name: OpcodeBaseName::BVS,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0x90 => Opcode {
            base_name: OpcodeBaseName::BCC,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0xB0 => Opcode {
            base_name: OpcodeBaseName::BCS,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0xD0 => Opcode {
            base_name: OpcodeBaseName::BNE,
            addressing: Addressing::Relative,
            cycle: 2,
        },
        0xF0 => Opcode {
            base_name: OpcodeBaseName::BEQ,
            addressing: Addressing::Relative,
            cycle: 2,
        },

        0xF8 => Opcode {
            base_name: OpcodeBaseName::SED,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xD8 => Opcode {
            base_name: OpcodeBaseName::CLD,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x1A => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x3A => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x5A => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x7A => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xDA => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xFA => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x02 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x12 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x22 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x32 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x42 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x52 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x62 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x72 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x92 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xB2 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xD2 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0xF2 => Opcode {
            base_name: OpcodeBaseName::NOP,
            addressing: Addressing::Implied,
            cycle: 2,
        },
        0x80 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0x82 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0x89 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xC2 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xE2 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Immediate,
            cycle: 3,
        },
        0x04 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 3,
        },
        0x44 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 3,
        },
        0x64 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 3,
        },
        0x14 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x34 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x54 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x74 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0xD4 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0xF4 => Opcode {
            base_name: OpcodeBaseName::NOPD,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x0C => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x1C => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x3C => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x5C => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0x7C => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0xDC => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0xFC => Opcode {
            base_name: OpcodeBaseName::NOPI,
            addressing: Addressing::Implied,
            cycle: 4,
        },
        0xA7 => Opcode {
            base_name: OpcodeBaseName::LAX,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0xB7 => Opcode {
            base_name: OpcodeBaseName::LAX,
            addressing: Addressing::ZeroPageY,
            cycle: 4,
        },
        0xAF => Opcode {
            base_name: OpcodeBaseName::LAX,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0xBF => Opcode {
            base_name: OpcodeBaseName::LAX,
            addressing: Addressing::AbsoluteY,
            cycle: 4,
        },
        0xA3 => Opcode {
            base_name: OpcodeBaseName::LAX,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0xB3 => Opcode {
            base_name: OpcodeBaseName::LAX,
            addressing: Addressing::IndirectY,
            cycle: 5,
        },
        0x87 => Opcode {
            base_name: OpcodeBaseName::SAX,
            addressing: Addressing::ZeroPage,
            cycle: 3,
        },
        0x97 => Opcode {
            base_name: OpcodeBaseName::SAX,
            addressing: Addressing::ZeroPageY,
            cycle: 4,
        },
        0x8F => Opcode {
            base_name: OpcodeBaseName::SAX,
            addressing: Addressing::Absolute,
            cycle: 4,
        },
        0x83 => Opcode {
            base_name: OpcodeBaseName::SAX,
            addressing: Addressing::IndirectX,
            cycle: 6,
        },
        0xEB => Opcode {
            base_name: OpcodeBaseName::SBC,
            addressing: Addressing::Immediate,
            cycle: 2,
        },
        0xC7 => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0xD7 => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0xCF => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0xDF => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0xDB => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::AbsoluteY,
            cycle: 7,
        },
        0xC3 => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::IndirectX,
            cycle: 8,
        },
        0xD3 => Opcode {
            base_name: OpcodeBaseName::DCP,
            addressing: Addressing::IndirectY,
            cycle: 8,
        },
        0xE7 => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0xF7 => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0xEF => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0xFF => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0xFB => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::AbsoluteY,
            cycle: 7,
        },
        0xE3 => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::IndirectX,
            cycle: 8,
        },
        0xF3 => Opcode {
            base_name: OpcodeBaseName::ISB,
            addressing: Addressing::IndirectY,
            cycle: 8,
        },
        0x07 => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x17 => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x0F => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x1F => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0x1B => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::AbsoluteY,
            cycle: 7,
        },
        0x03 => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::IndirectX,
            cycle: 8,
        },
        0x13 => Opcode {
            base_name: OpcodeBaseName::SLO,
            addressing: Addressing::IndirectY,
            cycle: 8,
        },
        0x27 => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x37 => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x2F => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x3F => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0x3B => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::AbsoluteY,
            cycle: 7,
        },
        0x23 => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::IndirectX,
            cycle: 8,
        },
        0x33 => Opcode {
            base_name: OpcodeBaseName::RLA,
            addressing: Addressing::IndirectY,
            cycle: 8,
        },
        0x47 => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x57 => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x4F => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x5F => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0x5B => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::AbsoluteY,
            cycle: 7,
        },
        0x43 => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::IndirectX,
            cycle: 8,
        },
        0x53 => Opcode {
            base_name: OpcodeBaseName::SRE,
            addressing: Addressing::IndirectY,
            cycle: 8,
        },
        0x67 => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::ZeroPage,
            cycle: 5,
        },
        0x77 => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::ZeroPageX,
            cycle: 6,
        },
        0x6F => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::Absolute,
            cycle: 6,
        },
        0x7F => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::AbsoluteX,
            cycle: 7,
        },
        0x7B => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::AbsoluteY,
            cycle: 7,
        },
        0x63 => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::IndirectX,
            cycle: 8,
        },
        0x73 => Opcode {
            base_name: OpcodeBaseName::RRA,
            addressing: Addressing::IndirectY,
            cycle: 8,
        },
        _ => {
            log(&format!("Unknown opcode: {:02X}", byte));
            panic!();
        }
    }
}
