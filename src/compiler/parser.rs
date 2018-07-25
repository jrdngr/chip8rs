use crate::compiler::tokenizer::Token;
use crate::cpu::opcode::OpCode;

pub fn parse(tokens: Vec<Token>) -> Vec<u8> {
    let mut result = Vec::new();

    let mut token_iter = tokens.into_iter();

    while let Some(token) = token_iter.next() {

    }

    result
}

fn split_u16(n: u16) -> (u8, u8) {
    
}