use std::result;

use instruction::{ArithmeticTarget, Instruction, JumpTest, LoadType, StackTarget};

mod instruction;

fn main() {
    
}

struct Registers{
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

impl std::convert:: From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister{
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0x01) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0x01) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0x01) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0x01) != 0;

        FlagsRegister{
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug, PartialEq)]
struct FlagsRegister{
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Registers{
    fn new() -> Self{
        Registers { a: 0, b: 0, c: 0, d: 0, e: 0, f: FlagsRegister { zero: false, subtract: false, half_carry: false, carry: false }, h: 0, l: 0 }
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | u8::from(self.f) as u16
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }

    fn set_af(&mut self, value: u16){
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn set_de(&mut self, value: u16){
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn set_hl(&mut self, value: u16){
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}


struct CPU{
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    fn new() -> Self{
        CPU{
            registers: Registers::new(),
            pc: 0x0000,
            sp: 0x0000,
            bus: MemoryBus::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => self.pc,
            Instruction::LD(load_type) => self.ld(load_type),
            Instruction::INC(target) => self.inc(target),
            Instruction::DEC(target) => self.dec(target),
            Instruction::ADD(target) => self.add(target),
            Instruction::JP(test) => self.jump(test),
            Instruction::JPHL => self.jumphl(),
            Instruction::PUSH(target) => self.push(target),
            Instruction::POP(target) => self.pop(target),
            Instruction::CALL(test) => self.call(test),
            Instruction::RET(test) => self.return_(test),
            Instruction::ADDHL(target) => self.addhl(target),
            _ => { panic!("TODO: support more instructions")}
        }
    }

    fn ld(&mut self, load_type: LoadType) -> u16{
        match load_type {
            LoadType::Byte(target, source) => {
                let source_value = self.read_registers_arithmeticTarget(source);
                self.change_registers_arithmeticTarget(target, source_value);
                
                match source {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            },
            LoadType::WORD(target, source) => {
                let source_value = match source {
                    ArithmeticTarget::D16 => self.read_next_word(),
                    _ => { panic!("TODO: inplement other sources")}
                };
                match target {
                    ArithmeticTarget::BC => self.registers.set_bc(source_value),
                    ArithmeticTarget::DE => self.registers.set_de(source_value),
                    ArithmeticTarget::HL => self.registers.set_hl(source_value),
                    _ => { panic!("TODO: inplement other targets")}
                };
                self.pc.wrapping_add(3)
            }
            _ => { panic!("TODO: inplement other load types")}
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
                    self.registers.f.carry);
                    self.change_registers_arithmeticTarget(target, new_value as u16);
                },

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
                    self.registers.f.carry);
                    self.change_registers_arithmeticTarget(target, new_value as u16);
                },

            false => {
                let (new_value, did_overflow) = value.overflowing_sub(1);
                self.change_registers_arithmeticTarget(target, new_value);
            }
        };
        self.pc.wrapping_add(1)
    }

    fn add (&mut self, target: ArithmeticTarget) -> u16 {
        match target {
            ArithmeticTarget::SP => {
                let r = (self.sp & 0x00FF) as u8;
                let value = self.read_next_byte() as i8;
                let (new_value, did_overflow) = self.sp.overflowing_add(value as u16);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.carry = did_overflow;
                self.registers.f.half_carry = (r & 0xF) + (value as u8 & 0xF) > 0xF;
                self.sp = new_value as u16;
                self.pc.wrapping_add(2)
            },
            _ => {
                let value = self.read_registers_arithmeticTarget(target) as u8;
                let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = did_overflow;
                self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
                self.registers.a = new_value;

                match target {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
            }
        }
        
    }

    fn addhl(&mut self, target: ArithmeticTarget) -> u16 {
        let value = self.read_registers_arithmeticTarget(target);
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
        self.registers.set_hl(new_value);
        self.pc.wrapping_add(1)
    }

    fn jump(&self, test: JumpTest) -> u16 {
        if (self.should_jump(test)) {
            let least_signigicant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_signigicant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn jumphl(&self) -> u16{
        self.registers.get_hl()
    }

    fn push(&mut self, target: StackTarget) -> u16 {
        let value = match target {
            StackTarget::AF => self.registers.get_af(),
            StackTarget::BC => self.registers.get_bc(),
            StackTarget::DE => self.registers.get_de(),
            StackTarget::HL => self.registers.get_hl(),
            StackTarget::D16(address) => address,
            _ => { panic!("TODO: support more targets") }
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
            StackTarget::AF => self.registers.set_af(result),
            StackTarget::BC => self.registers.set_bc(result),
            StackTarget::DE => self.registers.set_de(result),
            StackTarget::HL => self.registers.set_hl(result),
            StackTarget::NONE => (),
            _ => {panic!("TODO: support more targets")}
        }
        
        self.pc.wrapping_add(1)
    }

    fn call(&mut self, test: JumpTest) -> u16 {
        // self.call(jump_condition)
        let next_pc = self.pc.wrapping_add(3);

        if self.should_jump(test) {
            self.push(StackTarget::D16((next_pc)));
            self.pc.wrapping_sub(1);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, test: JumpTest) -> u16 {
        if self.should_jump(test) {
            self.pop(StackTarget::NONE)
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn step (&mut self){
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed){
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "CB" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for : 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
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
    fn read_registers_arithmeticTarget(&mut self, target: ArithmeticTarget) -> u16{
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
                self.registers.set_hl((self.registers.get_hl()).wrapping_add(1));
                value
            },
            ArithmeticTarget::HLd_ => {
                let value = self.bus.read_byte(self.registers.get_hl()) as u16;
                self.registers.set_hl((self.registers.get_hl()).wrapping_sub(1));
                value
            },
            ArithmeticTarget::BC_ => self.bus.read_byte(self.registers.get_bc()) as u16,
            ArithmeticTarget::DE_ => self.bus.read_byte(self.registers.get_de()) as u16,
            ArithmeticTarget::D8 => self.read_next_byte() as u16,
            ArithmeticTarget::D16_ => self.bus.read_byte(self.read_next_word()) as u16,
            //16bit
            ArithmeticTarget::BC => self.registers.get_bc(),
            ArithmeticTarget::DE => self.registers.get_de(),
            ArithmeticTarget::HL => self.registers.get_hl(),
            ArithmeticTarget::SP => self.sp,
            _ => panic!("TODO: support more targets")
        }
    }

    // レジスタやメモリへの代入を代行する
    fn change_registers_arithmeticTarget(&mut self, target: ArithmeticTarget, value: u16){
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
                self.registers.set_hl((self.registers.get_hl()).wrapping_add(1));
            },
            ArithmeticTarget::HLd_ => { 
                self.bus.write_byte(self.registers.get_hl(), value as u8); 
                self.registers.set_hl((self.registers.get_hl()).wrapping_sub(1));
            },
            // 16bit
            ArithmeticTarget::BC => self.registers.set_bc(value),
            ArithmeticTarget::DE => self.registers.set_de(value),
            ArithmeticTarget::HL => self.registers.set_hl(value),
            _ => panic!("TODO: support more targets")
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
            _ => panic!("TODO: support more targets")
        }
    }

    fn change_flag(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool){
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
            JumpTest::Always => true
        }
    }

}

struct  MemoryBus{
    memory: [u8; 0xFFFF]
}

impl MemoryBus{
    fn new() -> Self{
        MemoryBus { memory: [0; 0xFFFF] }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8){
        self.memory[address as usize] = value;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn F(zero: bool, subtract: bool, half_carry: bool, carry: bool) -> FlagsRegister {
        FlagsRegister{
            zero,
            subtract,
            half_carry,
            carry
        }
    }

    #[test]
    fn test_inc(){
        // B
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x04);
        cpu.registers.b = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // B zero
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x04);
        cpu.registers.b = 0xFF;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(true, false, true, false)
        );

        // (HL)
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x34);
        cpu.bus.write_byte(0x1000, 0x00);
        cpu.registers.set_hl(0x1000);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1000), 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // BC
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x03);
        cpu.registers.set_bc(0x1000);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x1001);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // BC 8
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x03);
        cpu.registers.set_bc(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x0100);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // BC over
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x03);
        cpu.registers.set_bc(0xFFFF);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );
    }

    #[test]
    fn test_dec(){
        // B
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x05);
        cpu.registers.b = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, true, false, false)
        );

        // B zero
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x05);
        cpu.registers.b = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(true, true, false, false)
        );

        // (HL)
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x35);
        cpu.bus.write_byte(0x1000, 0x02);
        cpu.registers.set_hl(0x1000);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1000), 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, true, false, false)
        );

        // BC
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B);
        cpu.registers.set_bc(0x1002);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x1001);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // BC 8
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B);
        cpu.registers.set_bc(0x0100);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x00FF);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // BC over
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B);
        cpu.registers.set_bc(0x0000);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0xFFFF);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );
    }

    #[test]
    fn test_add_a(){
        // A, A
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x87);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x04);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, B
        cpu.bus.write_byte(0x0001, 0x80);
        cpu.registers.b = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, C
        cpu.bus.write_byte(0x0002, 0x81);
        cpu.registers.c = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, D
        cpu.bus.write_byte(0x0003, 0x82);
        cpu.registers.d = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0004);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, E
        cpu.bus.write_byte(0x0004, 0x83);
        cpu.registers.e = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0005);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, H
        cpu.bus.write_byte(0x0005, 0x84);
        cpu.registers.h = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0006);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, L
        cpu.bus.write_byte(0x0006, 0x85);
        cpu.registers.l = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0007);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, (HL)
        cpu.bus.write_byte(0x0007, 0x86);
        cpu.bus.write_byte(0x1000, 0x0A);
        cpu.registers.set_hl(0x1000);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x0C);
        assert_eq!(cpu.pc, 0x0008);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // A, D8
        cpu.bus.write_byte(0x0008, 0xC6);
        cpu.bus.write_byte(0x0009, 0x03);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x000A);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        
    }

    #[test]
    fn test_add_sp(){
        // SP, D8
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8);
        cpu.bus.write_byte(0x0001, 0x03);
        cpu.sp = 0x0100;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0x0103);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // SP, D8 miner
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8);
        cpu.bus.write_byte(0x0001, 0xF0);
        cpu.sp = 0x0000;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0xFFF0);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );
        
        // SP, D8 carry
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8);
        cpu.bus.write_byte(0x0001, 0x10);
        cpu.sp = 0x00F0;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0x0100);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );
    }

    #[test]
    fn test_add_c(){
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        )
    }

    #[test]
    fn test_add_c_zero(){
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0x00;
        cpu.registers.a = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(true, false, false, false)
        )
    }

    #[test]
    fn test_add_c_carry(){
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0xF0;
        cpu.registers.a = 0x20;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, true)
        )
    }

    #[test]
    fn test_add_c_half_carry(){
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81);
        cpu.registers.c = 0x0F;
        cpu.registers.a = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, true, false)
        )
    }

    #[test]
    fn test_add_hl(){
        // HL, BC
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_bc(0x0005);
        cpu.registers.set_hl(0x0003);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0008);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // HL, DE
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x19);
        cpu.registers.set_de(0x0001);
        cpu.registers.set_hl(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0100);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // HL, HL
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x29);
        cpu.registers.set_hl(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x01FE);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // HL, SP
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x39);
        cpu.sp = 0x00FF;
        cpu.registers.set_hl(0x00FF);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x01FE);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, false)
        );

        // half carry
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_bc(0x0100);
        cpu.registers.set_hl(0x0F10);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x1010);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, true, false)
        );

        // carry
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_bc(0xF000);
        cpu.registers.set_hl(0x1000);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            F(false, false, false, true)
        );
    }

    #[test]
    fn test_jp(){
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
}