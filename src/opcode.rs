#[derive(Debug, PartialEq)]
pub enum OpCode {
    ExecuteMachineSubroutine(usize),// 0NNN
    ClearScreen,                    // 00E0
    ReturnFromSubroutine,           // 00EE
    JumpTo(usize),                  // 1NNN
    ExecuteSubroutine(usize),       // 2NNN
    SkipIfEqualValue(u8, u8),       // 3XNN
    SkipIfNotEqualValue(u8, u8),    // 4XNN
    SkipIfEqualRegister(u8, u8),    // 5XY0
    StoreValue(u8, u8),             // 6XNN
    AddValue(u8, u8),               // 7XNN
    StoreRegister(u8, u8),          // 8XY0
    Or(u8, u8),                     // 8XY1
    And(u8, u8),                    // 8XY2
    Xor(u8, u8),                    // 8XY3
    AddRegister(u8, u8),            // 8XY4
    SubtractRegister(u8, u8),       // 8XY5
    ShiftRight(u8, u8),             // 8XY6
    SubtractRegisterReverse(u8, u8),// 8XY7
    ShiftLeft(u8, u8),              // 8XYE
    SkipIfNotEqualRegister(u8, u8), // 9XY0
    StoreInI(usize),                // ANNN
    JumpWithOffset(usize),          // BNNN
    SetToRandom(u8, u8),            // CXNN
    DrawSprite(u8, u8, u8),         // DXYN
    SkipIfKeyPressed(u8),           // EX9E
    SkipIfKeyNotPressed(u8),        // EXA1
    StoreDelayTimer(u8),            // FX07
    WaitAndStoreKey(u8),            // FX0A
    SetDelayTimer(u8),              // FX15
    SetSoundTimer(u8),              // FX18
    AddToRegisterI(u8),             // FX1E
    SetIToHexSprite(u8),            // FX29
    StoreDecimal(u8),               // FX33
    StoreRegisters(u8),             // FX55
    FillRegisters(u8),              // FX65
}

impl From<u16> for OpCode {
    fn from(value: u16) -> Self {
        let (path, x, y, n) = get_hex_digits(value);
        match path {
            0x0 => {
                match value {
                    0x00E0 => OpCode::ClearScreen,
                    0x00EE => OpCode::ReturnFromSubroutine,
                    _ => OpCode::ExecuteMachineSubroutine(get_last_12_bits(value)),
                }
            },
            0x1 => OpCode::JumpTo(get_last_12_bits(value)),
            0x2 => OpCode::ExecuteSubroutine(get_last_12_bits(value)),
            0x3 => OpCode::SkipIfEqualValue(x, get_last_byte(value)),
            0x4 => OpCode::SkipIfNotEqualValue(x, get_last_byte(value)),
            0x5 => if n == 0 {
                OpCode::SkipIfEqualRegister(x, y)
            } else {
                panic!("{} is not a valid opcode", value)
            },
            0x6 => OpCode::StoreValue(x, get_last_byte(value)),
            0x7 => OpCode::AddValue(x, get_last_byte(value)),
            0x8 => match n {
                0x0 => OpCode::StoreRegister(x, y),
                0x1 => OpCode::Or(x, y),
                0x2 => OpCode::And(x, y),
                0x3 => OpCode::Xor(x, y),
                0x4 => OpCode::AddRegister(x, y),
                0x5 => OpCode::SubtractRegister(x, y),
                0x6 => OpCode::ShiftRight(x, y),
                0x7 => OpCode::SubtractRegisterReverse(x, y),
                0xE => OpCode::ShiftLeft(x, y),
                _ => panic!("{} is not a valid opcode", value),
            },
            0x9 => OpCode::SkipIfNotEqualRegister(x, y),
            0xA => OpCode::StoreInI(get_last_12_bits(value)),
            0xB => OpCode::JumpWithOffset(get_last_12_bits(value)),
            0xC => OpCode::SetToRandom(x, get_last_byte(value)),
            0xD => OpCode::DrawSprite(x, y, n),
            0xE => match get_last_byte(value) {
                0x9E => OpCode::SkipIfKeyPressed(x),
                0xA1 => OpCode::SkipIfKeyNotPressed(x),
                _ => panic!("{} is not a valid opcode", value),
            },
            0xF => match get_last_byte(value) {
                0x07 => OpCode::StoreDelayTimer(x),
                0x0A => OpCode::WaitAndStoreKey(x),
                0x15 => OpCode::SetDelayTimer(x),
                0x18 => OpCode::SetSoundTimer(x),
                0x1E => OpCode::AddToRegisterI(x),
                0x29 => OpCode::SetIToHexSprite(x),
                0x33 => OpCode::StoreDecimal(x),
                0x55 => OpCode::StoreRegisters(x),
                0x65 => OpCode::FillRegisters(x),
                _ => panic!("{} is not a valid opcode", value),
            },
            _ => panic!("{} is not a valid opcode", value),
        }
    }
}

fn get_hex_digits(value: u16) -> (u8, u8, u8, u8) {
    (((value & 0xF000) >> 12) as u8, ((value & 0x0F00) >> 8) as u8, ((value & 0x00F0) >> 4) as u8, (value & 0x000F) as u8)
}

fn get_first_byte(value: u16) -> u8 {
    ((value & 0xFF00) >> 8) as u8
}

fn get_last_byte(value: u16) -> u8 {
    (value & 0x00FF) as u8
}

fn get_last_12_bits(value: u16) -> usize {
    (value & 0x0FFF) as usize
}


#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_opcodes() {
        assert_eq!(OpCode::from(0x01B3), OpCode::ExecuteMachineSubroutine(0x1B3));
        assert_eq!(OpCode::from(0x00E0), OpCode::ClearScreen);
        assert_eq!(OpCode::from(0x00EE), OpCode::ReturnFromSubroutine);
        assert_eq!(OpCode::from(0x1123), OpCode::JumpTo(0x123));
        assert_eq!(OpCode::from(0x23B2), OpCode::ExecuteSubroutine(0x3B2));
        assert_eq!(OpCode::from(0x3123), OpCode::SkipIfEqualValue(1, 0x23));
        assert_eq!(OpCode::from(0x4123), OpCode::SkipIfNotEqualValue(1, 0x23));
        assert_eq!(OpCode::from(0x5120), OpCode::SkipIfEqualRegister(1, 2));
        assert_eq!(OpCode::from(0x6123), OpCode::StoreValue(1, 0x23));
        assert_eq!(OpCode::from(0x7123), OpCode::AddValue(1, 0x23));
        assert_eq!(OpCode::from(0x8120), OpCode::StoreRegister(1, 2));
        assert_eq!(OpCode::from(0x8121), OpCode::Or(1, 2));
        assert_eq!(OpCode::from(0x8122), OpCode::And(1, 2));
        assert_eq!(OpCode::from(0x8123), OpCode::Xor(1, 2));
        assert_eq!(OpCode::from(0x8124), OpCode::AddRegister(1, 2));
        assert_eq!(OpCode::from(0x8125), OpCode::SubtractRegister(1, 2));
        assert_eq!(OpCode::from(0x8126), OpCode::ShiftRight(1, 2));
        assert_eq!(OpCode::from(0x8127), OpCode::SubtractRegisterReverse(1, 2));
        assert_eq!(OpCode::from(0x812E), OpCode::ShiftLeft(1, 2));
        assert_eq!(OpCode::from(0x9120), OpCode::SkipIfNotEqualRegister(1, 2));
        assert_eq!(OpCode::from(0xA123), OpCode::StoreInI(0x123));
        assert_eq!(OpCode::from(0xB123), OpCode::JumpWithOffset(0x123));
        assert_eq!(OpCode::from(0xC123), OpCode::SetToRandom(1, 0x23));
        assert_eq!(OpCode::from(0xD123), OpCode::DrawSprite(1, 2, 3));
        assert_eq!(OpCode::from(0xE19E), OpCode::SkipIfKeyPressed(1));
        assert_eq!(OpCode::from(0xE1A1), OpCode::SkipIfKeyNotPressed(1));
        assert_eq!(OpCode::from(0xF107), OpCode::StoreDelayTimer(1));
        assert_eq!(OpCode::from(0xF10A), OpCode::WaitAndStoreKey(1));
        assert_eq!(OpCode::from(0xF115), OpCode::SetDelayTimer(1));
        assert_eq!(OpCode::from(0xF118), OpCode::SetSoundTimer(1));
        assert_eq!(OpCode::from(0xF11E), OpCode::AddToRegisterI(1));
        assert_eq!(OpCode::from(0xF129), OpCode::SetIToHexSprite(1));
        assert_eq!(OpCode::from(0xF133), OpCode::StoreDecimal(1));
        assert_eq!(OpCode::from(0xF155), OpCode::StoreRegisters(1));
        assert_eq!(OpCode::from(0xF165), OpCode::FillRegisters(1));
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcodes() {
        let op = OpCode::from(0x5EE1);
    }

    #[test]
    fn test_hex_digits() {
        assert_eq!(get_hex_digits(0x12AB), (0x1, 0x2, 0xA, 0xB));
    }
}