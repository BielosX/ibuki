use crate::draw::Draw;
use crate::raw_canvas::{PixelColor, RawCanvas};

pub struct Point2d {
    pub x: f32,
    pub y: f32
}

impl Point2d {
    pub fn new(x: f32, y: f32) -> Point2d {
        Point2d { x, y }
    }
}

impl Draw for Point2d {
    fn draw(&self, canvas: &RawCanvas) {
        let x = self.x as u32;
        let y = self.y as u32;
        canvas.put_pixel(x, y, &PixelColor::red());
    }
}