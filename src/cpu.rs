use instruction::{ArithmeticTarget, Instruction, JumpTest, LoadType, StackTarget};
use memory_bus::MemoryBus;

use crate::{
    cartridge::{self, Cartridge},
    instruction, memory_bus,
};

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0x01) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0x01) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0x01) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0x01) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Registers {
    fn new() -> Self {
        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0x01,
            l: 0x4D,
        }
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

const CYCLE: [u16; 256] = [
    4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4, 
    4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    12, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    12, 12, 8, 8, 12, 12, 12, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    8, 8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    20, 12, 16, 16, 24, 16, 8, 16, 20, 16, 16, 4, 24, 24, 8, 16,
    20, 12, 16, 4, 24, 16, 8, 16, 20, 16, 16, 4, 24, 4, 8, 16,
    12, 12, 8, 4, 4, 16, 8, 16, 16, 4, 16, 4, 4, 4, 8, 16,
    12, 12, 8, 4, 4, 16, 8, 16, 12, 8, 16, 4, 4, 4, 8, 16,
];

const CYCLE_2: [u16; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    8, 0, 12, 0, 12, 0, 0, 0, 8, 0, 12, 0, 12, 0, 0, 0,
    8, 0, 12, 0, 12, 0, 0, 0, 8, 0, 12, 0, 12, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const CYCLE_PREFIXED: [u16; 256] = [
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8,
    8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8,
    8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8,
    8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
];

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub is_halted: bool,
    cycle2_flag: bool,
    pub ime: bool,
}

impl CPU {
    pub fn new(cartridge: Cartridge) -> Self {
        CPU {
            registers: Registers::new(),
            pc: 0x0100,
            sp: 0xFFFE,
            bus: MemoryBus::new(cartridge),
            is_halted: false,
            cycle2_flag: false,
            ime: false,
        }
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::LD(load_type) => self.ld(load_type),
            Instruction::INC(target) => self.inc(target),
            Instruction::DEC(target) => self.dec(target),
            Instruction::ADD(target) => self.add(target),
            Instruction::ADDHL(target) => self.addhl(target),
            Instruction::ADC(target) => self.adc(target),
            Instruction::SUB(target) => self.sub(target),
            Instruction::SBC(target) => self.sub(target),
            Instruction::CP(target) => self.cp(target),
            Instruction::DAA => self.daa(),
            Instruction::AND(target) => self.and(target),
            Instruction::OR(target) => self.or(target),
            Instruction::XOR(target) => self.xor(target),
            Instruction::RR(target) => self.rr(target),
            Instruction::RRA => self.rra(),
            Instruction::RRC(target) => self.rrc(target),
            Instruction::RRCA => self.rrca(),
            Instruction::RL(target) => self.rl(target),
            Instruction::RLA => self.rla(),
            Instruction::RLC(target) => self.rlc(target),
            Instruction::RLCA => self.rlca(),
            Instruction::SLA(target) => self.sla(target),
            Instruction::SRA(target) => self.sra(target),
            Instruction::SWAP(target) => self.swap(target),
            Instruction::SRL(target) => self.srl(target),
            Instruction::BIT(target, n) => self.bit(target, n),
            Instruction::RES(target, n) => self.res(target, n),
            Instruction::SET(target, n) => self.set(target, n),
            Instruction::CPL => self.cpl(),
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::JP(test) => self.jump(test),
            Instruction::JPHL => self.jumphl(),
            Instruction::JR(test) => self.jr(test),
            Instruction::PUSH(target) => self.push(target),
            Instruction::POP(target) => self.pop(target),
            Instruction::CALL(test) => self.call(test),
            Instruction::RST(address) => self.rst(address),
            Instruction::RET(test) => self.ret(test),
            Instruction::RETI => self.reti(),
            Instruction::STOP => self.pc.wrapping_add(1), //TODO
            Instruction::HALT => self.pc.wrapping_add(1), //TODO
            Instruction::DI => self.di(),
            Instruction::EI => self.ei(),
            // _ => { panic!("TODO: support more instructions")}
        }
    }

    fn ld(&mut self, load_type: LoadType) -> u16 {
        match load_type {
            LoadType::Byte(target, source) => {
                let source_value = self.read_registers_arithmeticTarget(source);
                self.change_registers_arithmeticTarget(target, source_value);

                match source {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            LoadType::WORD(target, source) => {
                let source_value = self.read_registers_arithmeticTarget(source);
                match target {
                    ArithmeticTarget::SPA => {
                        let r = (self.sp & 0x00FF) as u8;
                        let value = self.read_next_byte() as i8;
                        let (new_value, did_overflow) = self.sp.overflowing_add(value as u16);
                        self.registers.f.zero = false;
                        self.registers.f.subtract = false;
                        self.registers.f.carry = did_overflow;
                        self.registers.f.half_carry = (r & 0xF) + (value as u8 & 0xF) > 0xF;
                        self.change_registers_arithmeticTarget(target, new_value);
                        self.pc.wrapping_add(2)
                    }
                    _ => {
                        self.change_registers_arithmeticTarget(target, source_value);
                        self.pc.wrapping_add(3)
                    }
                }
            }
            _ => {
                panic!("TODO: inplement other load types")
            }
        }
    }

    fn inc(&mut self, target: ArithmeticTarget) -> u16 {
        let is8bit = self.is_8bit(target);
        let value = self.read_registers_arithmeticTarget(target);
        match is8bit {
            true => {
                let (new_value, did_overflow) = (value as u8).overflowing_add(1);
                self.change_flag(
                    new_value == 0,
                    false,
                    value & 0x0F == 0x0F,
                    self.registers.f.carry,
                );
                self.change_registers_arithmeticTarget(target, new_value as u16);
            }

            false => {
                let (new_value, did_overflow) = value.overflowing_add(1);
                self.change_registers_arithmeticTarget(target, new_value);
            }
        };
        self.pc.wrapping_add(1)
    }

    fn dec(&mut self, target: ArithmeticTarget) -> u16 {
        let is8bit = self.is_8bit(target);
        let value = self.read_registers_arithmeticTarget(target);
        match is8bit {
            true => {
                let (new_value, did_overflow) = (value as u8).overflowing_sub(1);
                self.change_flag(
                    new_value == 0,
                    true,
                    value & 0x0F == 0x00,
                    self.registers.f.carry,
                );
                self.change_registers_arithmeticTarget(target, new_value as u16);
            }

            false => {
                let (new_value, did_overflow) = value.overflowing_sub(1);
                self.change_registers_arithmeticTarget(target, new_value);
            }
        };
        self.pc.wrapping_add(1)
    }

    fn add(&mut self, target: ArithmeticTarget) -> u16 {
        match target {
            ArithmeticTarget::SP => {
                let r = (self.sp & 0x00FF) as u8;
                let value = self.read_next_byte() as i8;
                let (new_value, did_overflow) = self.sp.overflowing_add(value as u16);
                self.change_flag(
                    false,
                    false,
                    (r & 0xF) + (value as u8 & 0xF) > 0xF,
                    did_overflow,
                );

                self.sp = new_value as u16;
                self.pc.wrapping_add(2)
            }
            _ => {
                let value = self.read_registers_arithmeticTarget(target) as u8;
                let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
                self.change_flag(
                    new_value == 0,
                    false,
                    (self.registers.a & 0xF) + (value & 0xF) > 0xF,
                    did_overflow,
                );

                self.registers.a = new_value;

                match target {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
        }
    }

    fn adc(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        let carry_inc: u8 = if self.registers.f.carry { 1 } else { 0 };
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value + carry_inc);
        self.change_flag(
            new_value == 0,
            false,
            (self.registers.a & 0xF) + (value & 0xF) + carry_inc > 0xF,
            did_overflow,
        );

        self.registers.a = new_value;

        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn addhl(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target);
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry =
            (self.registers.get_hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
        self.registers.set_hl(new_value);
        self.pc.wrapping_add(1)
    }

    fn sub(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.change_flag(
            new_value == 0,
            false,
            (self.registers.a & 0x0F) < 0x0F + (value & 0x0F),
            did_overflow,
        );

        self.registers.a = new_value;
        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn sbc(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        let carry_inc: u8 = if self.registers.f.carry { 1 } else { 0 };
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value + carry_inc);
        self.change_flag(
            new_value == 0,
            false,
            (self.registers.a & 0x0F) < 0x0F + (value & 0x0F) + carry_inc,
            did_overflow,
        );

        self.registers.a = new_value;
        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn cp(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.change_flag(
            new_value == 0,
            false,
            (self.registers.a & 0x0F) < 0x0F + (value & 0x0F),
            did_overflow,
        );

        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn daa(&mut self) -> u16 {
        self.pc.wrapping_add(1)
    }

    fn and(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        self.registers.a &= value;
        self.change_flag(self.registers.a == 0, false, true, false);
        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn or(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        self.registers.a |= value;
        self.change_flag(self.registers.a == 0, false, false, false);
        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn xor(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        self.registers.a ^= value;
        self.change_flag(self.registers.a == 0, false, false, false);
        match target {
            ArithmeticTarget::D8 => self.pc.wrapping_add(2),
            _ => self.pc.wrapping_add(1),
        }
    }

    fn rr(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let next_carry = self.get_bit(value, 0);
        value >>= 1;
        if self.registers.f.carry {
            value = self.set_bit(value, 7)
        };
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rra(&mut self) -> u16 {
        let mut value = self.registers.a;
        let next_carry = self.get_bit(value, 0);
        value >>= 1;
        if self.registers.f.carry {
            value = self.set_bit(value, 7)
        };
        self.registers.a = value;
        self.change_flag(false, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rrc(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let next_carry = self.get_bit(value, 0);
        value >>= 1;
        if next_carry {
            value = self.set_bit(value, 7)
        };
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rrca(&mut self) -> u16 {
        let mut value = self.registers.a;
        let next_carry = self.get_bit(value, 0);
        value >>= 1;
        if next_carry {
            value = self.set_bit(value, 7)
        };
        self.registers.a = value;
        self.change_flag(false, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rl(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let next_carry = self.get_bit(value, 7);
        value <<= 1;
        if self.registers.f.carry {
            value = self.set_bit(value, 0);
        };
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rla(&mut self) -> u16 {
        let mut value = self.registers.a;
        let next_carry = self.get_bit(value, 7);
        value <<= 1;
        if self.registers.f.carry {
            value = self.set_bit(value, 0);
        };
        self.registers.a = value;
        self.change_flag(false, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rlc(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let next_carry = self.get_bit(value, 7);
        value <<= 1;
        if next_carry {
            value = self.set_bit(value, 0);
        };
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn rlca(&mut self) -> u16 {
        let mut value = self.registers.a;
        let next_carry = self.get_bit(value, 7);
        value <<= 1;
        if next_carry {
            value = self.set_bit(value, 0);
        };
        self.registers.a = value;
        self.change_flag(false, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn sla(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let next_carry = self.get_bit(value, 7);
        value <<= 1;
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn sra(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let bit_7 = self.get_bit(value, 7);
        let next_carry = self.get_bit(value, 0);
        value >>= 1;
        if bit_7 {
            value = self.set_bit(value, 7);
        };
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn swap(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        value = value >> 4 | value << 4;
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, false);
        self.pc.wrapping_add(1)
    }

    fn srl(&mut self, target: ArithmeticTarget) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        let next_carry = self.get_bit(value, 0);
        value >>= 1;
        self.change_registers_arithmeticTarget(target, value as u16);
        self.change_flag(value == 0, false, false, next_carry);
        self.pc.wrapping_add(1)
    }

    fn bit(&mut self, target: ArithmeticTarget, n: u8) -> u16 {
        let value = self.read_registers_arithmeticTarget(target) as u8;
        self.change_flag(!self.get_bit(value, n), false, true, self.registers.f.carry);
        self.pc.wrapping_add(1)
    }

    fn res(&mut self, target: ArithmeticTarget, n: u8) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        value = self.res_bit(value, n);
        self.change_registers_arithmeticTarget(target, value as u16);
        self.pc.wrapping_add(1)
    }

    fn set(&mut self, target: ArithmeticTarget, n: u8) -> u16 {
        let mut value = self.read_registers_arithmeticTarget(target) as u8;
        value = self.set_bit(value, n);
        self.change_registers_arithmeticTarget(target, value as u16);
        self.pc.wrapping_add(1)
    }

    fn cpl(&mut self) -> u16 {
        self.registers.a ^= 0xFF;
        self.change_flag(self.registers.f.zero, true, true, self.registers.f.carry);
        self.pc.wrapping_add(1)
    }

    fn ccf(&mut self) -> u16 {
        self.change_flag(self.registers.f.zero, false, false, !self.registers.f.carry);
        self.pc.wrapping_add(1)
    }

    fn scf(&mut self) -> u16 {
        self.change_flag(self.registers.f.zero, false, false, true);
        self.pc.wrapping_add(1)
    }

    fn jump(&mut self, test: JumpTest) -> u16 {
        if self.should_jump(test) {
            self.read_next_word()
        } else {
            self.cycle2_flag = true;
            self.pc.wrapping_add(3)
        }
    }

    fn jumphl(&self) -> u16 {
        self.registers.get_hl()
    }

    fn jr(&mut self, test: JumpTest) -> u16 {
        if self.should_jump(test) {
            let value = self.read_next_byte() as i8;
            self.pc.wrapping_add(2).wrapping_add(value as u16)
        } else {
            self.cycle2_flag = true;
            self.pc.wrapping_add(2)
        }
    }

    fn push(&mut self, target: StackTarget) -> u16 {
        let value = match target {
            StackTarget::AF => self.registers.get_af(),
            StackTarget::BC => self.registers.get_bc(),
            StackTarget::DE => self.registers.get_de(),
            StackTarget::HL => self.registers.get_hl(),
            StackTarget::D16(address) => address,
            _ => {
                panic!("TODO: support more targets")
            }
        };
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
        self.pc.wrapping_add(1)
    }

    fn pop(&mut self, target: StackTarget) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let result = (msb << 8) | lsb;

        match target {
            StackTarget::NONE => result,
            _ => {
                match target {
                    StackTarget::AF => self.registers.set_af(result),
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    _ => panic!("TODO: support more targets"),
                };
                self.pc.wrapping_add(1)
            }
        }
    }

    fn call(&mut self, test: JumpTest) -> u16 {
        // self.call(jump_condition)
        let next_pc = self.pc.wrapping_add(3);

        if self.should_jump(test) {
            self.push(StackTarget::D16(next_pc));
            self.read_next_word()
        } else {
            self.cycle2_flag = true;
            next_pc
        }
    }

    fn rst(&mut self, address: u16) -> u16 {
        let next_pc = self.pc.wrapping_add(3);

        self.push(StackTarget::D16(next_pc));
        address
    }

    fn ret(&mut self, test: JumpTest) -> u16 {
        if self.should_jump(test) {
            self.pop(StackTarget::NONE)
        } else {
            self.cycle2_flag = true;
            self.pc.wrapping_add(1)
        }
    }

    fn reti(&mut self) -> u16 {
        self.pop(StackTarget::NONE)
    }

    fn halt(&mut self) -> u16 {
        self.is_halted = true;
        self.pc.wrapping_add(1)
    }

    fn di(&mut self) -> u16 {
        self.ime = false;
        self.pc.wrapping_add(1)
    }

    fn ei(&mut self) -> u16 {
        self.ime = true;
        self.pc.wrapping_add(1)
    }

    pub fn step(&mut self) {
        if self.is_halted {
            return;
        }

        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            self.pc = self.pc.wrapping_add(1);
            instruction_byte = self.bus.read_byte(self.pc);
        }

        self.cycle2_flag = false;
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            let result = self.execute(instruction);
            let mut cycles = CYCLE[instruction_byte as usize];
            if self.cycle2_flag {
                cycles = CYCLE_2[instruction_byte as usize]
            } else if prefixed {
                cycles = CYCLE_PREFIXED[instruction_byte as usize];
            }
            self.bus.gpu.update(cycles);
            result
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "CB" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for : 0x{:x}", instruction_byte);
        };

        // println!("pc:0x{:02X?}", self.pc);
        self.pc = next_pc;
        self.bus.gpu.update(1);
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        let l = self.bus.read_byte(self.pc + 1) as u16;
        let u = self.bus.read_byte(self.pc + 2) as u16;
        (u << 8) | l
    }

    //レジスタやメモリの値を持ってくる
    fn read_registers_arithmeticTarget(&mut self, target: ArithmeticTarget) -> u16 {
        match target {
            ArithmeticTarget::A => self.registers.a as u16,
            ArithmeticTarget::B => self.registers.b as u16,
            ArithmeticTarget::C => self.registers.c as u16,
            ArithmeticTarget::D => self.registers.d as u16,
            ArithmeticTarget::E => self.registers.e as u16,
            ArithmeticTarget::H => self.registers.h as u16,
            ArithmeticTarget::L => self.registers.l as u16,
            ArithmeticTarget::HL_ => self.bus.read_byte(self.registers.get_hl()) as u16,
            ArithmeticTarget::HLi_ => {
                let value = self.bus.read_byte(self.registers.get_hl()) as u16;
                self.registers
                    .set_hl((self.registers.get_hl()).wrapping_add(1));
                value
            }
            ArithmeticTarget::HLd_ => {
                let value = self.bus.read_byte(self.registers.get_hl()) as u16;
                self.registers
                    .set_hl((self.registers.get_hl()).wrapping_sub(1));
                value
            }
            ArithmeticTarget::BC_ => self.bus.read_byte(self.registers.get_bc()) as u16,
            ArithmeticTarget::DE_ => self.bus.read_byte(self.registers.get_de()) as u16,
            ArithmeticTarget::D8 => self.read_next_byte() as u16,
            ArithmeticTarget::D16_ => self.bus.read_byte(self.read_next_word()) as u16,
            ArithmeticTarget::FD8_ => {
                self.bus.read_byte(0xFF00 + self.read_next_byte() as u16) as u16
            }
            ArithmeticTarget::FDC_ => self.bus.read_byte(0xFF00 + self.registers.c as u16) as u16,

            //16bit
            ArithmeticTarget::BC => self.registers.get_bc(),
            ArithmeticTarget::DE => self.registers.get_de(),
            ArithmeticTarget::HL => self.registers.get_hl(),
            ArithmeticTarget::SP => self.sp,
            ArithmeticTarget::D16 => self.read_next_word(),
            ArithmeticTarget::SPA => self.sp,
            // _ => panic!("TODO: support more targets")
        }
    }

    // レジスタやメモリへの代入を代行する
    fn change_registers_arithmeticTarget(&mut self, target: ArithmeticTarget, value: u16) {
        match target {
            ArithmeticTarget::A => self.registers.a = value as u8,
            ArithmeticTarget::B => self.registers.b = value as u8,
            ArithmeticTarget::C => self.registers.c = value as u8,
            ArithmeticTarget::D => self.registers.d = value as u8,
            ArithmeticTarget::E => self.registers.e = value as u8,
            ArithmeticTarget::H => self.registers.h = value as u8,
            ArithmeticTarget::L => self.registers.l = value as u8,
            ArithmeticTarget::BC_ => self.bus.write_byte(self.registers.get_bc(), value as u8),
            ArithmeticTarget::DE_ => self.bus.write_byte(self.registers.get_de(), value as u8),
            ArithmeticTarget::HL_ => self.bus.write_byte(self.registers.get_hl(), value as u8),
            ArithmeticTarget::HLi_ => {
                self.bus.write_byte(self.registers.get_hl(), value as u8);
                self.registers
                    .set_hl((self.registers.get_hl()).wrapping_add(1));
            }
            ArithmeticTarget::HLd_ => {
                self.bus.write_byte(self.registers.get_hl(), value as u8);
                self.registers
                    .set_hl((self.registers.get_hl()).wrapping_sub(1));
            }
            ArithmeticTarget::D16_ => {
                self.bus.write_byte(self.read_next_word(), value as u8);
            },
            ArithmeticTarget::FD8_ => {
                self.bus.write_byte(0xFF00 + self.read_next_byte() as u16, value as u8);
            },
            ArithmeticTarget::FDC_ => {
                self.bus.write_byte(0xFF00 + self.registers.c as u16, value as u8);
            }
            // 16bit
            ArithmeticTarget::BC => self.registers.set_bc(value),
            ArithmeticTarget::DE => self.registers.set_de(value),
            ArithmeticTarget::HL => self.registers.set_hl(value),
            ArithmeticTarget::SP => self.sp = value,
            _ => panic!("TODO: support more targets"),
        };
    }

    fn is_8bit(&mut self, target: ArithmeticTarget) -> bool {
        match target {
            ArithmeticTarget::A => true,
            ArithmeticTarget::B => true,
            ArithmeticTarget::C => true,
            ArithmeticTarget::D => true,
            ArithmeticTarget::E => true,
            ArithmeticTarget::H => true,
            ArithmeticTarget::L => true,
            ArithmeticTarget::D8 => true,
            ArithmeticTarget::BC_ => true,
            ArithmeticTarget::DE_ => true,
            ArithmeticTarget::HL_ => true,
            ArithmeticTarget::HLi_ => true,
            ArithmeticTarget::HLd_ => true,
            ArithmeticTarget::D16_ => true,
            ArithmeticTarget::FD8_ => true,
            ArithmeticTarget::FDC_ => true,

            ArithmeticTarget::BC => false,
            ArithmeticTarget::DE => false,
            ArithmeticTarget::HL => false,
            ArithmeticTarget::SP => false,
            ArithmeticTarget::D16 => false,
            ArithmeticTarget::SPA => false,
            _ => panic!("TODO: support more targets"),
        }
    }

    fn change_flag(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        self.registers.f.zero = zero;
        self.registers.f.subtract = subtract;
        self.registers.f.half_carry = half_carry;
        self.registers.f.carry = carry;
    }

    fn should_jump(&self, test: JumpTest) -> bool {
        match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true,
        }
    }

    fn get_bit(&self, value: u8, n: u8) -> bool {
        1 << n & value > 0
    }

    fn set_bit(&self, value: u8, n: u8) -> u8 {
        1 << n | value
    }

    fn res_bit(&self, value: u8, n: u8) -> u8 {
        let mut mask: u8 = 0x01 << n;
        mask ^= 0xFF;
        mask & value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn F(zero: bool, subtract: bool, half_carry: bool, carry: bool) -> FlagsRegister {
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }

    #[test]
    fn test_inc() {
        // B
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x04);
        cpu.registers.b = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // B zero
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x04);
        cpu.registers.b = 0xFF;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, false, true, false));

        // (HL)
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x34);
        cpu.bus.write_byte(0x1000, 0x00);
        cpu.registers.set_hl(0x1000);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1000), 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // BC
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x03);
        cpu.registers.set_bc(0x1000);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x1001);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // BC 8
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x03);
        cpu.registers.set_bc(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x0100);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // BC over
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x03);
        cpu.registers.set_bc(0xFFFF);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_dec() {
        // B
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x05);
        cpu.registers.b = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));

        // B zero
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x05);
        cpu.registers.b = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, true, false, false));

        // (HL)
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x35);
        cpu.bus.write_byte(0x1000, 0x02);
        cpu.registers.set_hl(0x1000);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1000), 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));

        // BC
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B);
        cpu.registers.set_bc(0x1002);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x1001);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // BC 8
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B);
        cpu.registers.set_bc(0x0100);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x00FF);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // BC over
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B);
        cpu.registers.set_bc(0x0000);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0xFFFF);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_a() {
        // A, A
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x87);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x04);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, B
        cpu.bus.write_byte(0x0001, 0x80);
        cpu.registers.b = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, C
        cpu.bus.write_byte(0x0002, 0x81);
        cpu.registers.c = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, D
        cpu.bus.write_byte(0x0003, 0x82);
        cpu.registers.d = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0004);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, E
        cpu.bus.write_byte(0x0004, 0x83);
        cpu.registers.e = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0005);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, H
        cpu.bus.write_byte(0x0005, 0x84);
        cpu.registers.h = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0006);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, L
        cpu.bus.write_byte(0x0006, 0x85);
        cpu.registers.l = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0007);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, (HL)
        cpu.bus.write_byte(0x0007, 0x86);
        cpu.bus.write_byte(0x1000, 0x0A);
        cpu.registers.set_hl(0x1000);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x0C);
        assert_eq!(cpu.pc, 0x0008);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, D8
        cpu.bus.write_byte(0x0008, 0xC6);
        cpu.bus.write_byte(0x0009, 0x03);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x000A);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_sp() {
        // SP, D8
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8);
        cpu.bus.write_byte(0x0001, 0x03);
        cpu.sp = 0x0100;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0x0103);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // SP, D8 miner
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8);
        cpu.bus.write_byte(0x0001, 0xF0);
        cpu.sp = 0x0000;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0xFFF0);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // SP, D8 carry
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8);
        cpu.bus.write_byte(0x0001, 0x10);
        cpu.sp = 0x00F0;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0x0100);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false))
    }

    #[test]
    fn test_add_c_zero() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0x00;
        cpu.registers.a = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, false, false, false))
    }

    #[test]
    fn test_add_c_carry() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0xF0;
        cpu.registers.a = 0x20;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, true))
    }

    #[test]
    fn test_add_c_half_carry() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0x0F;
        cpu.registers.a = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false))
    }

    #[test]
    fn test_add_hl() {
        // HL, BC
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_bc(0x0005);
        cpu.registers.set_hl(0x0003);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0008);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // HL, DE
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x19);
        cpu.registers.set_de(0x0001);
        cpu.registers.set_hl(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0100);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // HL, HL
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x29);
        cpu.registers.set_hl(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x01FE);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // HL, SP
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x39);
        cpu.sp = 0x00FF;
        cpu.registers.set_hl(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x01FE);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // half carry
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_bc(0x0100);
        cpu.registers.set_hl(0x0F10);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x1010);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));

        // carry
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_bc(0xF000);
        cpu.registers.set_hl(0x1000);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }

    #[test]
    fn test_or() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xB0);
        cpu.registers.b = 0x0F;
        cpu.registers.a = 0x81;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x8F);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xB0);
        cpu.registers.b = 0x00;
        cpu.registers.a = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xF6);
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.registers.a = 0x80;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x81);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_jp() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xC3);
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.step();
        assert_eq!(cpu.pc, 0x0201);

        // JP HL
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xC3);
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.step();
        assert_eq!(cpu.pc, 0x0201);
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCD);
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.bus.write_byte(0x0201, 0xC9);
        cpu.step();
        assert_eq!(cpu.pc, 0x0201);
        cpu.step();
        assert_eq!(cpu.pc, 0x0003);

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCC);
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.bus.write_byte(0x0201, 0xC9);
        cpu.step();
        assert_eq!(cpu.pc, 0x0003);

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCC);
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.bus.write_byte(0x0201, 0xC9);
        cpu.registers.f.zero = true;
        cpu.step();
        assert_eq!(cpu.pc, 0x0201);
        cpu.step();
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn test_jr() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x18);
        cpu.bus.write_byte(0x0001, 0xF0);
        cpu.step();
        assert_eq!(cpu.pc, 0xFFF2);

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0xFFFE, 0x18);
        cpu.bus.write_byte(0xFFFF, 0x03);
        cpu.pc = 0xFFFE;
        cpu.step();
        assert_eq!(cpu.pc, 0x0003);

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x18);
        cpu.bus.write_byte(0x0001, 0x80);
        cpu.step();
        assert_eq!(cpu.pc, 0xFF82);

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x18);
        cpu.bus.write_byte(0x0001, 0xFF);
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x28);
        cpu.bus.write_byte(0x0001, 0x10);
        cpu.step();
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn test_rr() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x18);
        cpu.registers.b = 0x22;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x11);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x18);
        cpu.registers.b = 0x22;
        cpu.registers.f.carry = true;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x91);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x18);
        cpu.registers.b = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(true, false, false, true));

        // rra
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x1F);
        cpu.registers.a = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }

    #[test]
    fn test_rrc() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x08);
        cpu.registers.b = 0x22;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x11);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x08);
        cpu.registers.b = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x80);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, true));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x08);
        cpu.registers.b = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(true, false, false, false));

        //rrca
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0F);
        cpu.registers.a = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0F);
        cpu.registers.a = 0x81;
        cpu.step();
        assert_eq!(cpu.registers.a, 0xC0);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }

    #[test]
    fn test_bit_res_set() {
        // BIT
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x40);
        cpu.registers.b = 0x81;
        cpu.step();
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, true, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x48);
        cpu.registers.b = 0x81;
        cpu.step();
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(true, false, true, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x78);
        cpu.registers.b = 0x81;
        cpu.step();
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, true, false));

        // RES
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x80);
        cpu.registers.b = 0x81;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x80);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0xA0);
        cpu.registers.b = 0xFF;
        cpu.step();
        assert_eq!(cpu.registers.b, 0xEF);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0xB8);
        cpu.registers.b = 0x80;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // SET
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0xC0);
        cpu.registers.b = 0x80;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x81);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0xE0);
        cpu.registers.b = 0x80;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x90);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_srl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x38);
        cpu.registers.b = 0x7E;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x3F);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x38);
        cpu.registers.b = 0x7E;
        cpu.registers.f.carry = true;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x3F);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCB);
        cpu.bus.write_byte(0x0001, 0x38);
        cpu.registers.b = 0x81;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x40);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }
}
