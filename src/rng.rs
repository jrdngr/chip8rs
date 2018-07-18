pub fn generate_random(seed: i64) -> u8 {
    ((seed * 1103515245 + 12345) % 255) as u8
}