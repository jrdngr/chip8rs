use crate::compiler::tokenizer::Token;

#[derive(Debug)]
pub enum Instruction {
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE,
    SNE,
    LD,
    ADD,
    OR,
    AND,
    XOR,
    SUB,
    SHR,
    SUBN,
    SHL,
    RND,
    DRW,
    SKP,
    SKNP,
}

impl Instruction {
    pub fn required_parameters(&self) -> Vec<Token> {
        match self {
            CLS  => vec![],
            RET  => vec![],
            SYS  => vec![],
            JP   => vec![],
            CALL => vec![],
            SE   => vec![],
            SNE  => vec![],
            LD   => vec![],
            ADD  => vec![],
            OR   => vec![],
            AND  => vec![],
            XOR  => vec![],
            SUB  => vec![],
            SHR  => vec![],
            SUBN => vec![],
            SHL  => vec![],
            RND  => vec![],
            DRW  => vec![],
            SKP  => vec![],
            SKNP => vec![],
        }
    }
}

// 00E0 - CLS
// 00EE - RET
// 0nnn - SYS addr
// 1nnn - JP addr
// 2nnn - CALL addr
// 3xkk - SE Vx, byte
// 4xkk - SNE Vx, byte
// 5xy0 - SE Vx, Vy
// 6xkk - LD Vx, byte
// 7xkk - ADD Vx, byte
// 8xy0 - LD Vx, Vy
// 8xy1 - OR Vx, Vy
// 8xy2 - AND Vx, Vy
// 8xy3 - XOR Vx, Vy
// 8xy4 - ADD Vx, Vy
// 8xy5 - SUB Vx, Vy
// 8xy6 - SHR Vx {, Vy}
// 8xy7 - SUBN Vx, Vy
// 8xyE - SHL Vx {, Vy}
// 9xy0 - SNE Vx, Vy
// Annn - LD I, addr
// Bnnn - JP V0, addr
// Cxkk - RND Vx, byte
// Dxyn - DRW Vx, Vy, nibble
// Ex9E - SKP Vx
// ExA1 - SKNP Vx
// Fx07 - LD Vx, DT
// Fx0A - LD Vx, K
// Fx15 - LD DT, Vx
// Fx18 - LD ST, Vx
// Fx1E - ADD I, Vx
// Fx29 - LD F, Vx
// Fx33 - LD B, Vx
// Fx55 - LD [I], Vx
// Fx65 - LD Vx, [I]

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "cls"  => Ok(Instruction::CLS),
            "ret"  => Ok(Instruction::RET),
            "sys"  => Ok(Instruction::SYS),
            "jp"   => Ok(Instruction::JP),
            "call" => Ok(Instruction::CALL),
            "se"   => Ok(Instruction::SE),
            "sne"  => Ok(Instruction::SNE),
            "ld"   => Ok(Instruction::LD),
            "add"  => Ok(Instruction::ADD),
            "or"   => Ok(Instruction::OR),
            "and"  => Ok(Instruction::AND),
            "xor"  => Ok(Instruction::XOR),
            "sub"  => Ok(Instruction::SUB),
            "shr"  => Ok(Instruction::SHR),
            "subn" => Ok(Instruction::SUBN),
            "shl"  => Ok(Instruction::SHL),
            "rnd"  => Ok(Instruction::RND),
            "drw"  => Ok(Instruction::DRW),
            "skp"  => Ok(Instruction::SKP),
            "sknp" => Ok(Instruction::SKNP),
            _      => Err(String::from("Invalid instruction")),
        }
    }
}