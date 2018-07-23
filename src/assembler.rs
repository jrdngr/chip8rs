use super::cpu::opcode::{ OpCode };

pub fn assemble(assembly: &String) -> Vec<u8> {
    let mut result = Vec::new();
    for line in assembly.lines() {
        result.push(parse_line(line).0);
        result.push(parse_line(line).1);
    }
    result
}

fn parse_line(line: &str) -> (u8, u8) {
    
    for word in line.split_whitespace() {

    }
    (0, 0)
}