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

struct FlagsRegister{
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Registers{
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | self.f.into() as u16
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.e as u16) << 8
        | self.d as u16
    }

    fn get_hl(&self) -> u16 {
        (self.l as u16) << 8
        | self.h as u16
    }

    fn set_af(&mut self, value: u16){
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
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

enum Instruction {
    ADD(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    // ADDHL (HLに追加) - ターゲットがHLレジスタに追加されることを除いてADDと同様
    // ADC (キャリー付き加算) - ADDと同様ですが、キャリーフラグの値も数値に加算されます。
    // SUB (減算) - 特定のレジスタに格納されている値とAレジスタの値を減算します。
    // SBC (キャリー付き減算) - ADDと同様ですが、キャリーフラグの値も数値から減算されます。
    // AND (論理積) - 特定のレジスタの値とAレジスタの値に対してビットごとの論理積をとる
    // OR (論理和) - 特定のレジスタの値とAレジスタの値のビットごとの論理和をとる
    // XOR (論理排他的論理和) - 特定のレジスタの値とAレジスタの値のビットごとの排他的論理和を実行します。
    // CP (比較) - SUB と同様ですが、減算の結果は A に格納されません。
    // INC (増分) - 特定のレジスタの値を1ずつ増やす
    // DEC (デクリメント) - 特定のレジスタの値を1減らす
    // CCF (補数キャリーフラグ) - キャリーフラグの値を切り替える
    // SCF (キャリーフラグの設定) - キャリーフラグをtrueに設定する
    // RRA (Aレジスタを右に回転) - キャリーフラグを介してAレジスタを右にビット回転します。
    // RLA (Aレジスタを左回転) - キャリーフラグを介してAレジスタを左にビット回転します。
    // RRCA (A レジスタを右に回転) - A レジスタを右にビット回転 (キャリー フラグを経由しない)
    // RRLA (A レジスタを左に回転) - A レジスタを左にビット回転 (キャリー フラグを経由しない)
    // CPL (補数) - Aレジスタのすべてのビットを切り替える
    // BIT (ビットテスト) - 特定のレジスタの特定のビットが設定されているかどうかをテストします。
    // RESET（ビットリセット） - 特定のレジスタの特定のビットを0に設定する
    // SET (ビットセット) - 特定のレジスタの特定のビットを1に設定する
    // SRL (右シフト論理) - 特定のレジスタを1ビット右にシフトする
    // RR (右回転) - キャリーフラグを介して特定のレジスタを1ビット右に回転します。
    // RL (左回転) - キャリーフラグを介して特定のレジスタを1ビット左に回転します。
    // RRC (右回転) - 特定のレジスタを 1 ビット右に回転します (キャリー フラグを経由しない)
    // RLC (左回転) - 特定のレジスタを 1 ビット左に回転します (キャリー フラグを経由しない)
    // SRA (算術右シフト) - 特定のレジスタを 1 だけ右に算術シフトします。
    // SLA (算術左シフト) - 特定のレジスタを 1 左に算術シフトします。
    // SWAP (ニブルのスワップ) - 特定のレジスタの上位ニブルと下位ニブルを入れ替える
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

enum JumpTest{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

enum LoadType{
    Byte(LoadByteTarget, LoadByteSource)
}

enum StackTarget{
    AF, BC, DE, HL
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            _ => None,
        }
    }
}

struct CPU{
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

struct  MemoryBus{
    memory: [u8; 0xFFFF]
}

impl MemoryBus{
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8){
        self.memory[address as usize] = value;
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte((self.registers.get_hl())),
                            _ => { panic!("TODO: inplement other sources")}
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                            _ => { panic!("TODO: inplement other targets")}
                        };
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    _ => { panic!("TODO: inplement other load types")}
                }
            },
            Instruction::ADD(target) =>{
                match target {
                    ArithmeticTarget::A => self.pc,
                    ArithmeticTarget::B => self.pc,
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => self.pc,
                    ArithmeticTarget::E => self.pc,
                    ArithmeticTarget::H => self.pc,
                    ArithmeticTarget::L => self.pc,
                }
            },
            Instruction::JP(test) => {
                let jump_condition = match test{ 
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            },
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    _ => { panic!("TODO: support more targets") }
                };
                self.push(value);
                self.pc.wrapping_add(1)
            },
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::AF => self.registers.set_af(result),
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    _ => {panic!("TODO: support more targets")}
                }
            },
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                    _ => { panic!("TODO: support more conditions")}
                };
                self.call(jump_condition)
            },
            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                    _ => { panic!("TODO: support more conditions")}
                };
                self.return_(jump_condition)
            }
            _ => { panic!("TODO: support more instructions")}
        }
    }
    
    fn add (&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if (should_jump) {
            let least_signigicant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_signigicant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
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
}