use crate::compiler::instructions::Instruction;
use crate::compiler::tokenizer::Token;
use crate::cpu::opcode::OpCode;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();

    let mut token_iter = tokens.into_iter();

    let mut required_parameters: usize = 0;

    while let Some(token) = token_iter.next() {
        if required_parameters > 0 && !token.is_parameter() {
            return Err(String::from("Missing parameters"));
        }

        match token {
            Token::Constant => { required_parameters = 1; },
            Token::Number(n) => {},
            Token::Register(r) => {},
            Token::Instruction(i) => {},
            Token::LabelDefinition(l) => {},
            Token::LabelReference(l) => {},
        }
    }

    Ok(result)
}

fn split_u16(n: u16) -> (u8, u8) {
    ((n >> 8) as u8, (n & 0x00FF) as u8)
}
