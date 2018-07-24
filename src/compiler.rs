pub mod tokenizer;
pub mod parser;
pub mod assembler;

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    Ok(result)
}

pub fn clean_source(source: &str) -> String {
    strip_comments(source)
}

fn strip_comments(source: &str) -> String {
    let mut result = String::new();
    for line in source.lines() {
        let mut split_line: Vec<&str> = line.split(';').collect();
        if split_line.len() > 0 && split_line[0] != "" {
            result.push_str(split_line[0]);
            result.push('\n');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn test_strip_comments() {
        let mut f = File::open("breakout.src").expect("File not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Error reading file");
        
        //println!("{}", strip_comments(&contents));
    }
}
