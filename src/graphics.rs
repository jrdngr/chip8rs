const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [false; WIDTH * HEIGHT],
        }
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, num_bytes: u8) {
        let index = self.get_index(x, y);
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = false;
        }
    }

    fn get_index(&self, x: u8, y: u8) -> usize {
        (y as usize * WIDTH) + (x as usize % WIDTH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        let display = Screen::new();
        assert_eq!(display.get_index(0, 0), 0);
        assert_eq!(display.get_index(1, 0), 1);
        assert_eq!(display.get_index(0, 1), WIDTH);
        assert_eq!(display.get_index(10, 10), 650);
    }
}