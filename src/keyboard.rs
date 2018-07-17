struct Keyboard {
    state: u16,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { state: 0 }
    }

    pub fn key_down(&mut self, key: u8) {
        self.state = self.state | key_to_mask(key);
    }

    pub fn key_up(&mut self, key: u8) {
        self.state = self.state ^ (!key);
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        true
    }

    fn key_to_mask(key: u8) -> u16 {
        1u16 << (key as u16 & 0x0F)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_mask() {
        let keyb = Keyboard::new();
        println!("k: {}", Keyboard::key_to_mask(0xF));
    }
}
