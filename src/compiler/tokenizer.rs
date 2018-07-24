#[derive(Debug)]
pub enum Token {
    Instruction(String),
    Register(u8),
    Number(u16),
    Constant,
    LabelDefinition(String),
    LabelReference(String),
}

pub fn get_token(s: &str) -> Token {
    match s.to_lowercase().as_ref() {
        | "cls"
        | "ret"
        | "sys"
        | "jp"
        | "call"
        | "se"
        | "sne"
        | "ld"
        | "add"
        | "or"
        | "and"
        | "xor"
        | "sub"
        | "shr"
        | "subn"
        | "shl"
        | "rnd"
        | "drw"
        | "skp"
        | "sknp" => Token::Instruction(String::from(s)),
        "dw" => Token::Constant,
        s => match s.parse::<u16>() {
            Ok(n) => Token::Number(n),
            Err(e) => if s.ends_with(':') {
                Token::LabelDefinition(String::from(s))
            } else {
                Token::LabelReference(String::from(s))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    use super::*;
    
    #[test]
    fn test_tokenizer() {
        let mut f = File::open("breakout.src").expect("File not found");
        let mut source = String::new();
        f.read_to_string(&mut source).expect("Error reading file");
        source = crate::compiler::clean_source(&source);
        
        for word in source.split_whitespace() {
            println!("{:?}", get_token(word));
        }

    }
}
