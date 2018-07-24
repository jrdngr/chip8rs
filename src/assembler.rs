use std::collections::HashMap;

use super::cpu::opcode::OpCode;

pub struct Assembler {
    label_addresses: HashMap<String, u16>,
}

impl Assembler {
    pub fn new(assembly_code: &str) -> Self {
        Assembler {
            label_addresses: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, assembly: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut label_addresses: HashMap<&str, u16> = HashMap::new();
        for line in assembly.lines() {
            if line.starts_with(";") {
                continue;
            }
            let line_bytes = Assembler::parse_line(line);
            match line_bytes {
                Ok((b1, b2)) => {
                    result.push(b1);
                    result.push(b2);
                },
                Err(error) => return Err(error),
            }
        }
        Ok(result)
    }

    fn parse_line(line: &str) -> Result<(u8, u8), String> {
        if  line.ends_with(":") {

        }

        Ok((0, 0))
    }
}

