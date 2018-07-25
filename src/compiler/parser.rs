use crate::compiler::instructions::Instruction;
use crate::compiler::tokenizer::Token;
use crate::cpu::opcode::OpCode;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();

    let mut token_iter = tokens.into_iter();

    while let Some(token) = token_iter.next() {
        // match token {
        //     Instruction::Constant => {}
        // }
    }

    Ok(result)
}

fn split_u16(n: u16) -> (u8, u8) {
    ((n >> 8) as u8, (n & 0x00FF) as u8)
}


// pub enum Token {
//     Instruction(Instruction),
//     Register(u8),
//     Number(u16),
//     Constant,
//     LabelDefinition(String),
//     LabelReference(String),
// }