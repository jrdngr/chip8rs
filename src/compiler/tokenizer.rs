use std::str::FromStr;

use crate::compiler::instructions::Instruction;

#[derive(Debug)]
pub enum Token {
    Constant,
    Number(u16),
    Register(u8),
    Instruction(Instruction),
    LabelDefinition(String),
    LabelReference(String),
}

impl Token {
    pub fn is_parameter(&self) -> bool {
        match self {
            | Token::Number(_)
            | Token::Register(_)
            | Token::LabelReference(_) => true,

            | Token::Constant
            | Token::Instruction(_)
            | Token::LabelDefinition(_) => false,
        }
    }
}

pub fn get_token_stream(s: &str) -> Vec<Token> {
    let mut result = Vec::new();
    for word in s.split_whitespace() {
        result.push(get_token(word));
    }

    result
}

fn get_token(s: &str) -> Token {
    if let Ok(instruction) = Instruction::from_str(s) {
        return Token::Instruction(instruction);
    }

    if let Ok(n) = s.parse::<u16>() {
        return Token::Number(n);
    }

    if s.starts_with('#') {
        if let Ok(n) = u16::from_str_radix(s.trim_left_matches('#'), 16) {
            return Token::Number(n);
        }
    }

    if s.ends_with(':') {
        return Token::LabelDefinition(String::from(s));
    }

    Token::LabelReference(String::from(s))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    use super::*;
    
    #[test]
    #[ignore]
    fn test_get_token() {
        let mut f = File::open("breakout.src").expect("File not found");
        let mut source = String::new();
        f.read_to_string(&mut source).expect("Error reading file");
        source = crate::compiler::clean_source(&source);
        
        for word in source.split_whitespace() {
            println!("{:?}", get_token(word));
        }
    }

    #[test]
    #[ignore]
    fn test_get_token_stream() {
        let mut f = File::open("breakout.src").expect("File not found");
        let mut source = String::new();
        f.read_to_string(&mut source).expect("Error reading file");
        
        source = crate::compiler::clean_source(&source);
        let tokens = get_token_stream(&source);

        println!("{:?}", tokens);
    }
}
