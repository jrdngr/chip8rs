#[derive(Debug, PartialEq)]
pub enum OpCode {
    ExecuteMachineSubroutine(u16),  // 0NNN
    ClearScreen,                    // 00E0
    ReturnFromSubroutine,           // 00EE
    JumpTo(u16),                    // 1NNN
    ExecuteSubroutine(u16),         // 2NNN
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
    StoreInI(u16),                  // ANNN
    JumpWithOffset(u16),            // BNNN
    SetToRandom(u8, u8),            // CXNN
    DrawSprite(u8, u8, u8),         // DXYN
    SkipIfKeyPressed(u8),           // EX9E
    SkipIfKeyNotPressed(u8),        // EXA1
    StoreDelayTimer(u8),            // FX07
    WaitAndStoreKey(u8),            // FX0A
    SetDelayTimer(u8),              // FX15
    SetSoundTimer(u8),              // FX18
    AddToRegisterI(u8),             // FX1E
    SetIToSprite(u8),               // FX29
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
            0x5 => OpCode::SkipIfEqualRegister(x, y),
            0x6 => OpCode::StoreValue(x, get_last_byte(value)),
            0x7 => OpCode::AddValue(x, get_last_byte(value)),
            0x8 => match n {
                _ => panic!("{} is not a valid opcode", value),
            },
            0x9 => OpCode::SkipIfNotEqualRegister(x, y),
            0xA => OpCode::StoreInI(get_last_12_bits(value)),
            0xB => OpCode::JumpWithOffset(get_last_12_bits(value)),
            0xC => OpCode::SetToRandom(x, get_last_byte(value)),
            0xD => OpCode::DrawSprite(x, y, n),
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

fn get_last_12_bits(value: u16) -> u16 {
    value & 0x0FFF
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcodes() {
        assert_eq!(OpCode::from(0x00E0), OpCode::ClearScreen);
        assert_eq!(OpCode::from(0x00EE), OpCode::ReturnFromSubroutine);
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