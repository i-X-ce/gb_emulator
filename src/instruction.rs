

pub enum Instruction {
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

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub enum JumpTest{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum LoadType{
    Byte(LoadByteTarget, LoadByteSource)
}

pub enum StackTarget{
    AF, BC, DE, HL
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
            _ => None
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>{
        match byte {
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0xC3 => Some(Instruction::JP(JumpTest::Always)),
            _ => None,
        }
    }
}