use crate::draw::Draw;
use crate::pixel_color::PixelColor;
use crate::point2d::Point2d;
use crate::raw_canvas::RawCanvas;

#[derive(Copy, Clone)]
pub struct Line {
    pub first: Point2d,
    pub last: Point2d
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line {
            first: Point2d::new(x1, y1),
            last: Point2d::new(x2, y2)
        }
    }
}

impl Draw for Line {
    fn draw(&self, canvas: &RawCanvas) {
        let delta_y = self.last.y - self.first.y;
        let delta_x = self.last.x - self.first.x;
        let first_x = self.first.x as u32;
        let first_y = self.first.y as u32;
        let last_x = self.last.x as u32;
        let last_y = self.last.y as u32;
        let slope = delta_y / delta_x;
        let mut x = first_x;
        let mut y = first_y;
        if first_y == last_y {
            let left = if last_x > first_x {first_x} else {last_x};
            let right = if last_x > first_x {last_x} else {first_x};
            for x_pos in left..(right+1) {
                canvas.put_pixel(x_pos, y, &PixelColor::red());
            }
        } else if first_x == last_x {
            let bottom = if last_y > first_y {first_y} else {last_y};
            let top = if last_y > first_y {last_y} else {first_y};
            for y_pos in bottom..(top+1) {
                canvas.put_pixel(x, y_pos, &PixelColor::red());
            }
        } else {
            if slope > 1.0 {
                let mut denominator = 2.0 * delta_y - delta_x;
                for _ in 0..(last_y - first_y) {
                    canvas.put_pixel(x, y, &PixelColor::red());
                    if denominator >= 0.0 {
                        denominator -= 2.0 * delta_x;
                    } else {
                        x+= 1;
                        denominator += 2.0 * delta_y - 2.0 * delta_x;
                    }
                    y += 1;
                }
            } else if slope > 0.0 {
                let mut denominator = 2.0 * delta_y - delta_x;
                for _ in 0..(last_x - first_x) {
                    canvas.put_pixel(x, y, &PixelColor::red());
                    if denominator >= 0.0 {
                        y += 1;
                        denominator += 2.0 * delta_y - 2.0 * delta_x;
                    } else {
                        denominator += 2.0 * delta_y;
                    }
                    x += 1;
                }
            } else if slope < -1.0 {
                let mut denominator = delta_y + 2.0 * delta_x;
                for _ in 0..(first_y - last_y) {
                    canvas.put_pixel(x, y, &PixelColor::red());
                    if denominator >= 0.0 {
                        x += 1;
                        denominator += 2.0 * delta_y + 2.0 * delta_x;
                    } else {
                        denominator += 2.0 * delta_x;
                    }
                    y -= 1;
                }
            } else {
                let mut denominator = 2.0 * delta_y +  delta_x;
                for _ in 0..(last_x - first_x) {
                    canvas.put_pixel(x, y, &PixelColor::red());
                    if denominator >= 0.0 {
                        denominator += 2.0 * delta_y;
                    } else {
                        y -= 1;
                        denominator += 2.0 * delta_x;
                    }
                    x += 1;
                }
            }
        }
    }
}
