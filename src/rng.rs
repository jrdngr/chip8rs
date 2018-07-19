pub struct Rng {
    seed: i32,
}

impl Rng {
    pub fn new(seed: i32) -> Self{
        Rng { seed }
    }

    pub fn random_u8(&mut self) -> u8 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed % 255) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let mut rng = Rng::new(1);
        assert_eq!(rng.random_u8(), 45);
        assert_eq!(rng.random_u8(), 173);
        assert_eq!(rng.random_u8(), 34);
        assert_eq!(rng.random_u8(), 9);

        let mut rng = Rng::new(2);
        assert_eq!(rng.random_u8(), 240);
        assert_eq!(rng.random_u8(), 22);
        assert_eq!(rng.random_u8(), 146);
        assert_eq!(rng.random_u8(), 141);
    }
}