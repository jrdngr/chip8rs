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