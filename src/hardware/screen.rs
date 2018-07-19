pub trait Screen {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize, is_on: bool);
    fn get_pixel(&self, x: usize, y: usize) -> bool;
    fn get_pixels(&self) -> Vec<bool>;
}