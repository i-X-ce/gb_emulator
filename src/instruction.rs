

pub enum Instruction {
    NOP,
    LD(LoadType),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    CCF,
    SCF,
    DAA,
    JP(JumpTest),
    JPHL,
    JR(JumpTest),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RST(u16),
    RET(JumpTest),
    RETI,
    CPL,
    BIT(ArithmeticTarget, u8),
    RES(ArithmeticTarget, u8),
    SET(ArithmeticTarget, u8),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RRA,
    RL(ArithmeticTarget),
    RLA,
    RRC(ArithmeticTarget),
    RRCA,
    RLC(ArithmeticTarget),
    RLCA,
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    STOP,
    HALT,
    DI,
    EI,
}

#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
pub enum ArithmeticTarget { // XX_はインダイレクト, SPAはsp+D8
    A, B, C, D, E, H, L, BC_, DE_, HL_, HLi_, HLd_, BC, DE, HL, SP, D8, D16, D16_, FD8_, FDC_, SPA,
}

pub enum JumpTest{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum LoadType{
    Byte(ArithmeticTarget, ArithmeticTarget),
    WORD(ArithmeticTarget, ArithmeticTarget),
}

pub enum StackTarget{
    AF, BC, DE, HL, D16(u16), NONE
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
            0x01 => Some(Instruction::RLC(ArithmeticTarget::C)),
            0x02 => Some(Instruction::RLC(ArithmeticTarget::D)),
            0x03 => Some(Instruction::RLC(ArithmeticTarget::E)),
            0x04 => Some(Instruction::RLC(ArithmeticTarget::H)),
            0x05 => Some(Instruction::RLC(ArithmeticTarget::L)),
            0x06 => Some(Instruction::RLC(ArithmeticTarget::HL_)),
            0x07 => Some(Instruction::RLC(ArithmeticTarget::A)),

            0x08 => Some(Instruction::RRC(ArithmeticTarget::B)),
            0x09 => Some(Instruction::RRC(ArithmeticTarget::C)),
            0x0A => Some(Instruction::RRC(ArithmeticTarget::D)),
            0x0B => Some(Instruction::RRC(ArithmeticTarget::E)),
            0x0C => Some(Instruction::RRC(ArithmeticTarget::H)),
            0x0D => Some(Instruction::RRC(ArithmeticTarget::L)),
            0x0E => Some(Instruction::RRC(ArithmeticTarget::HL_)),
            0x0F => Some(Instruction::RRC(ArithmeticTarget::A)),
            
            0x10 => Some(Instruction::RL(ArithmeticTarget::B)),
            0x11 => Some(Instruction::RL(ArithmeticTarget::C)),
            0x12 => Some(Instruction::RL(ArithmeticTarget::D)),
            0x13 => Some(Instruction::RL(ArithmeticTarget::E)),
            0x14 => Some(Instruction::RL(ArithmeticTarget::H)),
            0x15 => Some(Instruction::RL(ArithmeticTarget::L)),
            0x16 => Some(Instruction::RL(ArithmeticTarget::HL_)),
            0x17 => Some(Instruction::RL(ArithmeticTarget::A)),
            
            0x18 => Some(Instruction::RR(ArithmeticTarget::B)),
            0x19 => Some(Instruction::RR(ArithmeticTarget::C)),
            0x1A => Some(Instruction::RR(ArithmeticTarget::D)),
            0x1B => Some(Instruction::RR(ArithmeticTarget::E)),
            0x1C => Some(Instruction::RR(ArithmeticTarget::H)),
            0x1D => Some(Instruction::RR(ArithmeticTarget::L)),
            0x1E => Some(Instruction::RR(ArithmeticTarget::HL_)),
            0x1F => Some(Instruction::RR(ArithmeticTarget::A)),
            
            0x20 => Some(Instruction::SLA(ArithmeticTarget::B)),
            0x21 => Some(Instruction::SLA(ArithmeticTarget::C)),
            0x22 => Some(Instruction::SLA(ArithmeticTarget::D)),
            0x23 => Some(Instruction::SLA(ArithmeticTarget::E)),
            0x24 => Some(Instruction::SLA(ArithmeticTarget::H)),
            0x25 => Some(Instruction::SLA(ArithmeticTarget::L)),
            0x26 => Some(Instruction::SLA(ArithmeticTarget::HL_)),
            0x27 => Some(Instruction::SLA(ArithmeticTarget::A)),
            
            0x28 => Some(Instruction::SRA(ArithmeticTarget::B)),
            0x29 => Some(Instruction::SRA(ArithmeticTarget::C)),
            0x2A => Some(Instruction::SRA(ArithmeticTarget::D)),
            0x2B => Some(Instruction::SRA(ArithmeticTarget::E)),
            0x2C => Some(Instruction::SRA(ArithmeticTarget::H)),
            0x2D => Some(Instruction::SRA(ArithmeticTarget::L)),
            0x2E => Some(Instruction::SRA(ArithmeticTarget::HL_)),
            0x2F => Some(Instruction::SRA(ArithmeticTarget::A)),
            
            0x30 => Some(Instruction::SWAP(ArithmeticTarget::B)),
            0x31 => Some(Instruction::SWAP(ArithmeticTarget::C)),
            0x32 => Some(Instruction::SWAP(ArithmeticTarget::D)),
            0x33 => Some(Instruction::SWAP(ArithmeticTarget::E)),
            0x34 => Some(Instruction::SWAP(ArithmeticTarget::H)),
            0x35 => Some(Instruction::SWAP(ArithmeticTarget::L)),
            0x36 => Some(Instruction::SWAP(ArithmeticTarget::HL_)),
            0x37 => Some(Instruction::SWAP(ArithmeticTarget::A)),
            
            0x38 => Some(Instruction::SRL(ArithmeticTarget::B)),
            0x39 => Some(Instruction::SRL(ArithmeticTarget::C)),
            0x3A => Some(Instruction::SRL(ArithmeticTarget::D)),
            0x3B => Some(Instruction::SRL(ArithmeticTarget::E)),
            0x3C => Some(Instruction::SRL(ArithmeticTarget::H)),
            0x3D => Some(Instruction::SRL(ArithmeticTarget::L)),
            0x3E => Some(Instruction::SRL(ArithmeticTarget::HL_)),
            0x3F => Some(Instruction::SRL(ArithmeticTarget::A)),
            
            0x40 => Some(Instruction::BIT(ArithmeticTarget::B, 0)),
            0x41 => Some(Instruction::BIT(ArithmeticTarget::C, 0)),
            0x42 => Some(Instruction::BIT(ArithmeticTarget::D, 0)),
            0x43 => Some(Instruction::BIT(ArithmeticTarget::E, 0)),
            0x44 => Some(Instruction::BIT(ArithmeticTarget::H, 0)),
            0x45 => Some(Instruction::BIT(ArithmeticTarget::L, 0)),
            0x46 => Some(Instruction::BIT(ArithmeticTarget::HL_, 0)),
            0x47 => Some(Instruction::BIT(ArithmeticTarget::A, 0)),
            0x48 => Some(Instruction::BIT(ArithmeticTarget::B, 1)),
            0x49 => Some(Instruction::BIT(ArithmeticTarget::C, 1)),
            0x4A => Some(Instruction::BIT(ArithmeticTarget::D, 1)),
            0x4B => Some(Instruction::BIT(ArithmeticTarget::E, 1)),
            0x4C => Some(Instruction::BIT(ArithmeticTarget::H, 1)),
            0x4D => Some(Instruction::BIT(ArithmeticTarget::L, 1)),
            0x4E => Some(Instruction::BIT(ArithmeticTarget::HL_, 1)),
            0x4F => Some(Instruction::BIT(ArithmeticTarget::A, 1)),
            0x50 => Some(Instruction::BIT(ArithmeticTarget::B, 2)),
            0x51 => Some(Instruction::BIT(ArithmeticTarget::C, 2)),
            0x52 => Some(Instruction::BIT(ArithmeticTarget::D, 2)),
            0x53 => Some(Instruction::BIT(ArithmeticTarget::E, 2)),
            0x54 => Some(Instruction::BIT(ArithmeticTarget::H, 2)),
            0x55 => Some(Instruction::BIT(ArithmeticTarget::L, 2)),
            0x56 => Some(Instruction::BIT(ArithmeticTarget::HL_, 2)),
            0x57 => Some(Instruction::BIT(ArithmeticTarget::A, 2)),
            0x58 => Some(Instruction::BIT(ArithmeticTarget::B, 3)),
            0x59 => Some(Instruction::BIT(ArithmeticTarget::C, 3)),
            0x5A => Some(Instruction::BIT(ArithmeticTarget::D, 3)),
            0x5B => Some(Instruction::BIT(ArithmeticTarget::E, 3)),
            0x5C => Some(Instruction::BIT(ArithmeticTarget::H, 3)),
            0x5D => Some(Instruction::BIT(ArithmeticTarget::L, 3)),
            0x5E => Some(Instruction::BIT(ArithmeticTarget::HL_, 3)),
            0x5F => Some(Instruction::BIT(ArithmeticTarget::A, 3)),
            0x60 => Some(Instruction::BIT(ArithmeticTarget::B, 4)),
            0x61 => Some(Instruction::BIT(ArithmeticTarget::C, 4)),
            0x62 => Some(Instruction::BIT(ArithmeticTarget::D, 4)),
            0x63 => Some(Instruction::BIT(ArithmeticTarget::E, 4)),
            0x64 => Some(Instruction::BIT(ArithmeticTarget::H, 4)),
            0x65 => Some(Instruction::BIT(ArithmeticTarget::L, 4)),
            0x66 => Some(Instruction::BIT(ArithmeticTarget::HL_, 4)),
            0x67 => Some(Instruction::BIT(ArithmeticTarget::A, 4)),
            0x68 => Some(Instruction::BIT(ArithmeticTarget::B, 5)),
            0x69 => Some(Instruction::BIT(ArithmeticTarget::C, 5)),
            0x6A => Some(Instruction::BIT(ArithmeticTarget::D, 5)),
            0x6B => Some(Instruction::BIT(ArithmeticTarget::E, 5)),
            0x6C => Some(Instruction::BIT(ArithmeticTarget::H, 5)),
            0x6D => Some(Instruction::BIT(ArithmeticTarget::L, 5)),
            0x6E => Some(Instruction::BIT(ArithmeticTarget::HL_, 5)),
            0x6F => Some(Instruction::BIT(ArithmeticTarget::A, 5)),
            0x70 => Some(Instruction::BIT(ArithmeticTarget::B, 6)),
            0x71 => Some(Instruction::BIT(ArithmeticTarget::C, 6)),
            0x72 => Some(Instruction::BIT(ArithmeticTarget::D, 6)),
            0x73 => Some(Instruction::BIT(ArithmeticTarget::E, 6)),
            0x74 => Some(Instruction::BIT(ArithmeticTarget::H, 6)),
            0x75 => Some(Instruction::BIT(ArithmeticTarget::L, 6)),
            0x76 => Some(Instruction::BIT(ArithmeticTarget::HL_, 6)),
            0x77 => Some(Instruction::BIT(ArithmeticTarget::A, 6)),
            0x78 => Some(Instruction::BIT(ArithmeticTarget::B, 7)),
            0x79 => Some(Instruction::BIT(ArithmeticTarget::C, 7)),
            0x7A => Some(Instruction::BIT(ArithmeticTarget::D, 7)),
            0x7B => Some(Instruction::BIT(ArithmeticTarget::E, 7)),
            0x7C => Some(Instruction::BIT(ArithmeticTarget::H, 7)),
            0x7D => Some(Instruction::BIT(ArithmeticTarget::L, 7)),
            0x7E => Some(Instruction::BIT(ArithmeticTarget::HL_, 7)),
            0x7F => Some(Instruction::BIT(ArithmeticTarget::A, 7)),
            
            0x80 => Some(Instruction::RES(ArithmeticTarget::B, 0)),
            0x81 => Some(Instruction::RES(ArithmeticTarget::C, 0)),
            0x82 => Some(Instruction::RES(ArithmeticTarget::D, 0)),
            0x83 => Some(Instruction::RES(ArithmeticTarget::E, 0)),
            0x84 => Some(Instruction::RES(ArithmeticTarget::H, 0)),
            0x85 => Some(Instruction::RES(ArithmeticTarget::L, 0)),
            0x86 => Some(Instruction::RES(ArithmeticTarget::HL_, 0)),
            0x87 => Some(Instruction::RES(ArithmeticTarget::A, 0)),
            0x88 => Some(Instruction::RES(ArithmeticTarget::B, 1)),
            0x89 => Some(Instruction::RES(ArithmeticTarget::C, 1)),
            0x8A => Some(Instruction::RES(ArithmeticTarget::D, 1)),
            0x8B => Some(Instruction::RES(ArithmeticTarget::E, 1)),
            0x8C => Some(Instruction::RES(ArithmeticTarget::H, 1)),
            0x8D => Some(Instruction::RES(ArithmeticTarget::L, 1)),
            0x8E => Some(Instruction::RES(ArithmeticTarget::HL_, 1)),
            0x8F => Some(Instruction::RES(ArithmeticTarget::A, 1)),
            0x90 => Some(Instruction::RES(ArithmeticTarget::B, 2)),
            0x91 => Some(Instruction::RES(ArithmeticTarget::C, 2)),
            0x92 => Some(Instruction::RES(ArithmeticTarget::D, 2)),
            0x93 => Some(Instruction::RES(ArithmeticTarget::E, 2)),
            0x94 => Some(Instruction::RES(ArithmeticTarget::H, 2)),
            0x95 => Some(Instruction::RES(ArithmeticTarget::L, 2)),
            0x96 => Some(Instruction::RES(ArithmeticTarget::HL_, 2)),
            0x97 => Some(Instruction::RES(ArithmeticTarget::A, 2)),
            0x98 => Some(Instruction::RES(ArithmeticTarget::B, 3)),
            0x99 => Some(Instruction::RES(ArithmeticTarget::C, 3)),
            0x9A => Some(Instruction::RES(ArithmeticTarget::D, 3)),
            0x9B => Some(Instruction::RES(ArithmeticTarget::E, 3)),
            0x9C => Some(Instruction::RES(ArithmeticTarget::H, 3)),
            0x9D => Some(Instruction::RES(ArithmeticTarget::L, 3)),
            0x9E => Some(Instruction::RES(ArithmeticTarget::HL_, 3)),
            0x9F => Some(Instruction::RES(ArithmeticTarget::A, 3)),
            0xA0 => Some(Instruction::RES(ArithmeticTarget::B, 4)),
            0xA1 => Some(Instruction::RES(ArithmeticTarget::C, 4)),
            0xA2 => Some(Instruction::RES(ArithmeticTarget::D, 4)),
            0xA3 => Some(Instruction::RES(ArithmeticTarget::E, 4)),
            0xA4 => Some(Instruction::RES(ArithmeticTarget::H, 4)),
            0xA5 => Some(Instruction::RES(ArithmeticTarget::L, 4)),
            0xA6 => Some(Instruction::RES(ArithmeticTarget::HL_, 4)),
            0xA7 => Some(Instruction::RES(ArithmeticTarget::A, 4)),
            0xA8 => Some(Instruction::RES(ArithmeticTarget::B, 5)),
            0xA9 => Some(Instruction::RES(ArithmeticTarget::C, 5)),
            0xAA => Some(Instruction::RES(ArithmeticTarget::D, 5)),
            0xAB => Some(Instruction::RES(ArithmeticTarget::E, 5)),
            0xAC => Some(Instruction::RES(ArithmeticTarget::H, 5)),
            0xAD => Some(Instruction::RES(ArithmeticTarget::L, 5)),
            0xAE => Some(Instruction::RES(ArithmeticTarget::HL_, 5)),
            0xAF => Some(Instruction::RES(ArithmeticTarget::A, 5)),
            0xB0 => Some(Instruction::RES(ArithmeticTarget::B, 6)),
            0xB1 => Some(Instruction::RES(ArithmeticTarget::C, 6)),
            0xB2 => Some(Instruction::RES(ArithmeticTarget::D, 6)),
            0xB3 => Some(Instruction::RES(ArithmeticTarget::E, 6)),
            0xB4 => Some(Instruction::RES(ArithmeticTarget::H, 6)),
            0xB5 => Some(Instruction::RES(ArithmeticTarget::L, 6)),
            0xB6 => Some(Instruction::RES(ArithmeticTarget::HL_, 6)),
            0xB7 => Some(Instruction::RES(ArithmeticTarget::A, 6)),
            0xB8 => Some(Instruction::RES(ArithmeticTarget::B, 7)),
            0xB9 => Some(Instruction::RES(ArithmeticTarget::C, 7)),
            0xBA => Some(Instruction::RES(ArithmeticTarget::D, 7)),
            0xBB => Some(Instruction::RES(ArithmeticTarget::E, 7)),
            0xBC => Some(Instruction::RES(ArithmeticTarget::H, 7)),
            0xBD => Some(Instruction::RES(ArithmeticTarget::L, 7)),
            0xBE => Some(Instruction::RES(ArithmeticTarget::HL_, 7)),
            0xBF => Some(Instruction::RES(ArithmeticTarget::A, 7)),
            
            0xC0 => Some(Instruction::SET(ArithmeticTarget::B, 0)),
            0xC1 => Some(Instruction::SET(ArithmeticTarget::C, 0)),
            0xC2 => Some(Instruction::SET(ArithmeticTarget::D, 0)),
            0xC3 => Some(Instruction::SET(ArithmeticTarget::E, 0)),
            0xC4 => Some(Instruction::SET(ArithmeticTarget::H, 0)),
            0xC5 => Some(Instruction::SET(ArithmeticTarget::L, 0)),
            0xC6 => Some(Instruction::SET(ArithmeticTarget::HL_, 0)),
            0xC7 => Some(Instruction::SET(ArithmeticTarget::A, 0)),
            0xC8 => Some(Instruction::SET(ArithmeticTarget::B, 1)),
            0xC9 => Some(Instruction::SET(ArithmeticTarget::C, 1)),
            0xCA => Some(Instruction::SET(ArithmeticTarget::D, 1)),
            0xCB => Some(Instruction::SET(ArithmeticTarget::E, 1)),
            0xCC => Some(Instruction::SET(ArithmeticTarget::H, 1)),
            0xCD => Some(Instruction::SET(ArithmeticTarget::L, 1)),
            0xCE => Some(Instruction::SET(ArithmeticTarget::HL_, 1)),
            0xCF => Some(Instruction::SET(ArithmeticTarget::A, 1)),
            0xD0 => Some(Instruction::SET(ArithmeticTarget::B, 2)),
            0xD1 => Some(Instruction::SET(ArithmeticTarget::C, 2)),
            0xD2 => Some(Instruction::SET(ArithmeticTarget::D, 2)),
            0xD3 => Some(Instruction::SET(ArithmeticTarget::E, 2)),
            0xD4 => Some(Instruction::SET(ArithmeticTarget::H, 2)),
            0xD5 => Some(Instruction::SET(ArithmeticTarget::L, 2)),
            0xD6 => Some(Instruction::SET(ArithmeticTarget::HL_, 2)),
            0xD7 => Some(Instruction::SET(ArithmeticTarget::A, 2)),
            0xD8 => Some(Instruction::SET(ArithmeticTarget::B, 3)),
            0xD9 => Some(Instruction::SET(ArithmeticTarget::C, 3)),
            0xDA => Some(Instruction::SET(ArithmeticTarget::D, 3)),
            0xDB => Some(Instruction::SET(ArithmeticTarget::E, 3)),
            0xDC => Some(Instruction::SET(ArithmeticTarget::H, 3)),
            0xDD => Some(Instruction::SET(ArithmeticTarget::L, 3)),
            0xDE => Some(Instruction::SET(ArithmeticTarget::HL_, 3)),
            0xDF => Some(Instruction::SET(ArithmeticTarget::A, 3)),
            0xE0 => Some(Instruction::SET(ArithmeticTarget::B, 4)),
            0xE1 => Some(Instruction::SET(ArithmeticTarget::C, 4)),
            0xE2 => Some(Instruction::SET(ArithmeticTarget::D, 4)),
            0xE3 => Some(Instruction::SET(ArithmeticTarget::E, 4)),
            0xE4 => Some(Instruction::SET(ArithmeticTarget::H, 4)),
            0xE5 => Some(Instruction::SET(ArithmeticTarget::L, 4)),
            0xE6 => Some(Instruction::SET(ArithmeticTarget::HL_, 4)),
            0xE7 => Some(Instruction::SET(ArithmeticTarget::A, 4)),
            0xE8 => Some(Instruction::SET(ArithmeticTarget::B, 5)),
            0xE9 => Some(Instruction::SET(ArithmeticTarget::C, 5)),
            0xEA => Some(Instruction::SET(ArithmeticTarget::D, 5)),
            0xEB => Some(Instruction::SET(ArithmeticTarget::E, 5)),
            0xEC => Some(Instruction::SET(ArithmeticTarget::H, 5)),
            0xED => Some(Instruction::SET(ArithmeticTarget::L, 5)),
            0xEE => Some(Instruction::SET(ArithmeticTarget::HL_, 5)),
            0xEF => Some(Instruction::SET(ArithmeticTarget::A, 5)),
            0xF0 => Some(Instruction::SET(ArithmeticTarget::B, 6)),
            0xF1 => Some(Instruction::SET(ArithmeticTarget::C, 6)),
            0xF2 => Some(Instruction::SET(ArithmeticTarget::D, 6)),
            0xF3 => Some(Instruction::SET(ArithmeticTarget::E, 6)),
            0xF4 => Some(Instruction::SET(ArithmeticTarget::H, 6)),
            0xF5 => Some(Instruction::SET(ArithmeticTarget::L, 6)),
            0xF6 => Some(Instruction::SET(ArithmeticTarget::HL_, 6)),
            0xF7 => Some(Instruction::SET(ArithmeticTarget::A, 6)),
            0xF8 => Some(Instruction::SET(ArithmeticTarget::B, 7)),
            0xF9 => Some(Instruction::SET(ArithmeticTarget::C, 7)),
            0xFA => Some(Instruction::SET(ArithmeticTarget::D, 7)),
            0xFB => Some(Instruction::SET(ArithmeticTarget::E, 7)),
            0xFC => Some(Instruction::SET(ArithmeticTarget::H, 7)),
            0xFD => Some(Instruction::SET(ArithmeticTarget::L, 7)),
            0xFE => Some(Instruction::SET(ArithmeticTarget::HL_, 7)),
            0xFF => Some(Instruction::SET(ArithmeticTarget::A, 7)),
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            0x00 => Some(Instruction::NOP),
            0x01 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::BC, ArithmeticTarget::D16))),
            0x02 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::BC_, ArithmeticTarget::A))),
            0x03 => Some(Instruction::INC(ArithmeticTarget::BC)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::D8))),
            0x07 => Some(Instruction::RLCA),
            0x08 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::D16_, ArithmeticTarget::SP))),
            0x09 => Some(Instruction::ADDHL(ArithmeticTarget::BC)),
            0x0A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::BC_))),
            0x0B => Some(Instruction::DEC(ArithmeticTarget::BC)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::D8))),
            0x0F => Some(Instruction::RRCA),
            
            0x10 => Some(Instruction::STOP),
            0x11 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::DE, ArithmeticTarget::D16))),
            0x12 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::DE_, ArithmeticTarget::A))),
            0x13 => Some(Instruction::INC(ArithmeticTarget::DE)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::D8))),
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR(JumpTest::Always)),
            0x19 => Some(Instruction::ADDHL(ArithmeticTarget::DE)),
            0x1A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::DE_))),
            0x1B => Some(Instruction::DEC(ArithmeticTarget::DE)),
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::D8))),
            0x1F => Some(Instruction::RRA),

            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x21 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::HL, ArithmeticTarget::D16))),
            0x22 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HLi_, ArithmeticTarget::A))),
            0x23 => Some(Instruction::INC(ArithmeticTarget::HL)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::D8))),
            0x27 => Some(Instruction::DAA),
            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x29 => Some(Instruction::ADDHL(ArithmeticTarget::HL)),
            0x2A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::HLi_))),
            0x2B => Some(Instruction::DEC(ArithmeticTarget::HL)),
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::D8))),
            0x2F => Some(Instruction::CPL),

            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x31 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::SP, ArithmeticTarget::D16))),
            0x32 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HLd_, ArithmeticTarget::A))),
            0x33 => Some(Instruction::INC(ArithmeticTarget::SP)),
            0x34 => Some(Instruction::INC(ArithmeticTarget::HL_)),
            0x35 => Some(Instruction::DEC(ArithmeticTarget::HL_)),
            0x36 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::D8))),
            0x37 => Some(Instruction::SCF),
            0x38 => Some(Instruction::JR(JumpTest::Carry)),
            0x39 => Some(Instruction::ADDHL(ArithmeticTarget::SP)),
            0x3A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::HLd_))),
            0x3B => Some(Instruction::DEC(ArithmeticTarget::SP)),
            0x3C => Some(Instruction::INC(ArithmeticTarget::A)),
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::D8))),
            0x3F => Some(Instruction::CCF),

            0x40 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::HL_))),
            0x47 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::B, ArithmeticTarget::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::HL_))),
            0x4F => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::C, ArithmeticTarget::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::HL_))),
            0x57 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D, ArithmeticTarget::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::HL_))),
            0x5F => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::E, ArithmeticTarget::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::HL_))),
            0x67 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::H, ArithmeticTarget::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::HL_))),
            0x6F => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::L, ArithmeticTarget::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::L))),
            0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::HL_, ArithmeticTarget::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::HL_))),
            0x7F => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::A))),

            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL_)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),

            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL_)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),

            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL_)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),

            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL_)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),

            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL_)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),

            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL_)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),

            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL_)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),

            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL_)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),

            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xC3 => Some(Instruction::JP(JumpTest::Always)),
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xC7 => Some(Instruction::RST(0x0000)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xCB => None,
            0xCD => Some(Instruction::CALL(JumpTest::Always)),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),
            0xCF => Some(Instruction::RST(0x0008)),

            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xD3 => None,
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),
            0xD7 => Some(Instruction::RST(0x0010)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xD9 => Some(Instruction::RETI),
            0xDA => Some(Instruction::JP(JumpTest::Carry)),
            0xDB => None,
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),
            0xDD => None,
            0xDE => Some(Instruction::SBC(ArithmeticTarget::D8)),
            0xDF => Some(Instruction::RST(0x0018)),

            0xE0 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::FD8_, ArithmeticTarget::A))),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::FDC_, ArithmeticTarget::A))),
            0xE3 => None,
            0xE4 => None,
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),
            0xE7 => Some(Instruction::RST(0x0020)),
            0xE8 => Some(Instruction::ADD(ArithmeticTarget::SP)),
            0xE9 => Some(Instruction::JPHL),
            0xEA => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::D16_, ArithmeticTarget::A))),
            0xEC => Some(Instruction::CALL(JumpTest::Carry)),
            0xED => None,
            0xEE => Some(Instruction::SBC(ArithmeticTarget::D8)),
            0xEF => Some(Instruction::RST(0x0028)),

            0xF0 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::FD8_))),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::FDC_))),
            0xF3 => Some(Instruction::DI),
            0xF4 => None,
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),
            0xF7 => Some(Instruction::RST(0x0030)),
            0xF8 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::HL, ArithmeticTarget::SPA))),
            0xF9 => Some(Instruction::LD(LoadType::WORD(ArithmeticTarget::SP, ArithmeticTarget::HL))),
            0xFA => Some(Instruction::LD(LoadType::Byte(ArithmeticTarget::A, ArithmeticTarget::D16_))),
            0xFC => None,
            0xFD => None,
            0xFE => Some(Instruction::CP(ArithmeticTarget::D8)),
            0xFF => Some(Instruction::RST(0x0038)),
            _ => None,
        }
    }
}