use crate::line::Line;
use crate::point2d::Point2d;

pub struct ClippingRectangle {
    y_min: f32,
    y_max: f32,
    x_min: f32,
    x_max: f32
}

impl ClippingRectangle {
    pub fn new(x_min: f32, y_min: f32, x_max: f32, y_max: f32) -> ClippingRectangle {
        ClippingRectangle { x_min, y_min, x_max, y_max }
    }
}

const TOP: u8 = 0x1;
const BOTTOM: u8 = 0x2;
const RIGHT: u8 = 0x4;
const LEFT: u8 = 0x8;

fn compute_out_code(point: &Point2d, clipping_rectangle: &ClippingRectangle) -> u8 {
    let mut result: u8 = 0;
    if point.y > clipping_rectangle.y_max {
        result |= TOP;
    } else if point.y < clipping_rectangle.y_min {
        result |= BOTTOM;
    }
    if point.x > clipping_rectangle.x_max {
        result |= RIGHT;
    } else if point.x < clipping_rectangle.x_min {
        result |= LEFT;
    }
    result
}

pub fn cohen_sutherland_line_clip(line: &Line, rectangle: &ClippingRectangle) -> Option<Line> {
    let mut first_out_code = compute_out_code(&line.first, rectangle);
    let mut last_out_code = compute_out_code(&line.last, rectangle);
    let mut done = false;
    let mut result = Some(line.clone());

    while !done {
        if (first_out_code | last_out_code) == 0 {
            done = true;
        } else if (first_out_code & last_out_code) != 0 {
            result = None;
            done = true;
        } else {
            let out_code = if first_out_code != 0 { first_out_code } else { last_out_code };
            let mut x: f32 = 0.0;
            let mut y: f32 = 0.0;
            if out_code & TOP != 0 {
                x = line.first.x + (line.last.x - line.first.x) * (rectangle.y_max - line.first.y) / (line.last.y - line.first.y);
                y = rectangle.y_max;
            } else if out_code & BOTTOM != 0 {
                x = line.first.x + (line.last.x - line.first.x) * (rectangle.y_min - line.first.y) / (line.last.y - line.first.y);
                y = rectangle.y_min;
            } else if out_code & RIGHT != 0 {
                y = line.first.y + (line.last.y - line.first.y) * (rectangle.x_max - line.first.x) / (line.last.x - line.first.x);
                x = rectangle.x_max;
            } else {
                y = line.first.y + (line.last.y - line.first.y) * (rectangle.x_min - line.first.x) / (line.last.x - line.first.x);
                x = rectangle.x_min;
            }

            if out_code == first_out_code {
                result = result.map(|l| Line::new(x, y, l.last.x, l.last.y));
                first_out_code = compute_out_code(&Point2d::new(x, y), rectangle);
            } else {
                result = result.map(|l| Line::new(l.first.x, l.first.y, x, y));
                last_out_code = compute_out_code(&Point2d::new(x, y), rectangle);
            }
        }
    }
    result
}