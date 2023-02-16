use crate::raw_canvas::RawCanvas;

pub trait Draw {
    fn draw(&self, canvas: &RawCanvas);
}