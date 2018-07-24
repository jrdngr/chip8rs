#[derive(Debug, PartialEq, Clone)]
pub enum OpCode {
    ExecuteMachineSubroutine(usize),        // 0NNN
    ClearScreen,                            // 00E0
    ReturnFromSubroutine,                   // 00EE
    JumpTo(usize),                          // 1NNN
    ExecuteSubroutine(usize),               // 2NNN
    SkipIfEqualValue(usize, u8),            // 3XNN
    SkipIfNotEqualValue(usize, u8),         // 4XNN
    SkipIfEqualRegister(usize, usize),      // 5XY0
    StoreValue(usize, u8),                  // 6XNN
    AddValue(usize, u8),                    // 7XNN
    StoreRegister(usize, usize),            // 8XY0
    Or(usize, usize),                       // 8XY1
    And(usize, usize),                      // 8XY2
    Xor(usize, usize),                      // 8XY3
    AddRegister(usize, usize),              // 8XY4
    SubtractRegister(usize, usize),         // 8XY5
    ShiftRight(usize, usize),               // 8XY6
    SubtractRegisterReverse(usize, usize),  // 8XY7
    ShiftLeft(usize, usize),                // 8XYE
    SkipIfNotEqualRegister(usize, usize),   // 9XY0
    StoreInI(usize),                        // ANNN
    JumpWithOffset(usize),                  // BNNN
    SetToRandom(usize, u8),                 // CXNN
    DrawSprite(usize, usize, u8),           // DXYN
    SkipIfKeyPressed(usize),                // EX9E
    SkipIfKeyNotPressed(usize),             // EXA1
    StoreDelayTimer(usize),                 // FX07
    WaitAndStoreKey(usize),                 // FX0A
    SetDelayTimer(usize),                   // FX15
    SetSoundTimer(usize),                   // FX18
    AddToRegisterI(usize),                  // FX1E
    SetIToHexSprite(usize),                 // FX29
    StoreDecimal(usize),                    // FX33
    StoreRegisters(usize),                  // FX55
    FillRegisters(usize),                   // FX65
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

impl std::str::FromStr for OpCode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut words = value.split_whitespace();
        if let Some(code) = words.next() {
            match code.to_lowercase().as_ref() {
                "cls" =>  return Ok(OpCode::ClearScreen),
                "ret" => return Ok(OpCode::ReturnFromSubroutine),
                _ => return Err(String::from(format!("{} is not a valid instruction", code))),
            };
        }
        Err(String::from("Invalid syntax"))
    }
}

fn get_hex_digits(value: u16) -> (u8, usize, usize, u8) {
    (((value & 0xF000) >> 12) as u8, ((value & 0x0F00) >> 8) as usize, ((value & 0x00F0) >> 4) as usize, (value & 0x000F) as u8)
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


/* Op Code Descriptions from http://mattmik.com/files/chip8/mastering/chip8.html
 *
 * 0NNN  Execute machine language subroutine at address NNN
 * 00E0  Clear the screen
 * 00EE  Return from a subroutine
 * 1NNN  Jump to address NNN
 * 2NNN  Execute subroutine starting at address NNN
 * 3XNN  Skip the following instruction if the value of register VX equals NN
 * 4XNN  Skip the following instruction if the value of register VX is not equal to NN
 * 5XY0  Skip the following instruction if the value of register VX is equal to the value of register VY
 * 6XNN  Store number NN in register VX
 * 7XNN  Add the value NN to register VX
 * 8XY0  Store the value of register VY in register VX
 * 8XY1  Set VX to VX OR VY
 * 8XY2  Set VX to VX AND VY
 * 8XY3  Set VX to VX XOR VY
 * 8XY4  Add the value of register VY to register VX
 *       Set VF to 01 if a carry occurs
 *       Set VF to 00 if a carry does not occur
 * 8XY5  Subtract the value of register VY from register VX
 *       Set VF to 00 if a borrow occurs
 *       Set VF to 01 if a borrow does not occur
 * 8XY6  Store the value of register VY shifted right one bit in register VX
 *       Set register VF to the least significant bit prior to the shift
 * 8XY7  Set register VX to the value of VY minus VX
 *       Set VF to 00 if a borrow occurs
 *       Set VF to 01 if a borrow does not occur
 * 8XYE  Store the value of register VY shifted left one bit in register VX
 *       Set register VF to the most significant bit prior to the shift
 * 9XY0  Skip the following instruction if the value of register VX is not equal to the value of register VY
 * ANNN  Store memory address NNN in register I
 * BNNN  Jump to address NNN + V0
 * CXNN  Set VX to a random number with a mask of NN
 * DXYN  Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
 *       Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
 * EX9E  Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
 * EXA1  Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
 * FX07  Store the current value of the delay timer in register VX
 * FX0A  Wait for a keypress and store the result in register VX
 * FX15  Set the delay timer to the value of register VX
 * FX18  Set the sound timer to the value of register VX
 * FX1E  Add the value stored in register VX to register I
 * FX29  Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
 * FX33  Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I+1, and I+2
 * FX55  Store the values of registers V0 to VX inclusive in memory starting at address I
 *       I is set to I + X + 1 after operation
 * FX65  Fill registers V0 to VX inclusive with the values stored in memory starting at address I
 *       I is set to I + X + 1 after operation
 * 
 */