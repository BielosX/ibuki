use float_cmp::approx_eq;
use sdl2::libc::clone;
use crate::line::Line;
use crate::point2d::Point2d;
use crate::polygon::Polygon;
use crate::vector2d::Vector2d;

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

fn line_passing_two_points_x(first: &Point2d, second: &Point2d, y: f32) -> f32 {
    first.x + (second.x - first.x) * (y - first.y) / (second.y - first.y)
}

fn line_passing_two_points_y(first: &Point2d, second: &Point2d, x: f32) -> f32 {
    first.y + (second.y - first.y) * (x - first.x) / (second.x - first.x)
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
                x = line_passing_two_points_x(&line.first, &line.last, rectangle.y_max);
                y = rectangle.y_max;
            } else if out_code & BOTTOM != 0 {
                x = line_passing_two_points_x(&line.first, &line.last, rectangle.y_min);
                y = rectangle.y_min;
            } else if out_code & RIGHT != 0 {
                y = line_passing_two_points_y(&line.first, &line.last, rectangle.x_max);
                x = rectangle.x_max;
            } else {
                y = line_passing_two_points_y(&line.first, &line.last, rectangle.x_min);
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

// Counter clockwise clipping_polygon edges
pub fn cyrus_beck_line_clip(line: &Line, clipping_polygon: &Vec<Point2d>) -> Option<Line> {
    let mut result = None;
    if line.first.x == line.last.x && line.first.y == line.last.y {
        result = Some(line.clone());
    } else {
        if clipping_polygon.len() >= 3 {
            let mut edges: Vec<Vector2d> = Vec::new();
            let length = clipping_polygon.len();
            for x in 0..length {
                let mut next_idx = if x == (length - 1) { 0 } else { x + 1 };
                let first = clipping_polygon.get(x).unwrap();
                let second = clipping_polygon.get(next_idx).unwrap();
                let edge = Vector2d::new(first.x, first.y, second.x, second.y);
                edges.push(edge);
            }
            let mut t_entering: f32 = 0.0;
            let mut t_leaving: f32 = 1.0;
            let segment_vector = Vector2d::from_2d_points(&line.first, &line.last);
            for edge in edges.iter() {
                let normal_left = edge.normal_left();
                let normal_dot_segment = normal_left.dot(&segment_vector);
                if !approx_eq!(f32, normal_dot_segment, 0.0) {
                    let p0_to_pei = Vector2d::from_2d_points(&edge.get_to(), &line.first);
                    let t = (normal_left.dot(&p0_to_pei)) / (- normal_dot_segment);
                    if normal_dot_segment < 0.0 { // Possibly Entering
                        t_entering = t_entering.max(t);
                    } else { // Possibly Leaving
                        t_leaving = t_leaving.min(t);
                    }
                }
            }
            if t_entering > t_leaving {
                result = None;
            } else {
                let p0 = Vector2d::new(0.0, 0.0, line.first.x, line.first.y);
                let t_entering_scaled = &segment_vector * t_entering;
                let t_leaving_scaled = &segment_vector * t_leaving;
                let result_from = (&p0 + &t_entering_scaled).get_to();
                let result_to = (&p0 + &t_leaving_scaled).get_to();
                result = Some(Line::new(result_from.x, result_from.y, result_to.x, result_to.y));
            }
        }
    }
    result
}

fn inside(vertex: &Point2d, clipping_edge: &Vector2d) -> bool {
    let edge_to_vertex = Vector2d::from_2d_points(&clipping_edge.get_from(), vertex).to_starting_zero();
    clipping_edge.normal_left().dot(&edge_to_vertex) < 0.0
}

fn intersect(polygon_edge: &Vector2d, clipping_edge: &Vector2d) -> Option<Point2d> {
    let normal_left = clipping_edge.normal_left();
    let denominator = -normal_left.dot(polygon_edge);
    if !approx_eq!(f32, denominator, 0.0) {
        let clipping_edge_point_to_polygon_edge_vertex = Vector2d::from_2d_points(&clipping_edge.get_to(), &polygon_edge.get_from());
        let nominator = normal_left.dot(&clipping_edge_point_to_polygon_edge_vertex);
        let t = nominator / denominator;
        let scaled_edge = polygon_edge.to_starting_zero() * t;
        let result = scaled_edge + Vector2d::from_2d_points(&Point2d::zero(), &polygon_edge.get_from());
        Some(result.get_to())
    } else {
        None
    }
}

fn sutherland_hodgman_polygon_edge_clip(polygon: &Polygon, clipping_edge: (&Point2d, &Point2d)) -> Option<Polygon> {
    let edge_vector = Vector2d::from_2d_points(clipping_edge.0, clipping_edge.1);
    let length = polygon.points.len();
    let mut result: Vec<Point2d> = Vec::new();
    for i in 0..length {
        let begin = polygon.points.get(i).unwrap();
        let end_index = if i == (length-1) {0} else {i+1};
        let end = polygon.points.get(end_index).unwrap();
        let polygon_edge_vector = Vector2d::from_2d_points(begin, end);
        if inside(end, &edge_vector) {
            if inside(begin, &edge_vector) {
                result.push(end.clone());
            } else {
                let intersection = intersect(&polygon_edge_vector, &edge_vector);
                match intersection {
                    None => {
                        result.push(end.clone());
                    }
                    Some(p) => {
                        result.push(p);
                        result.push(end.clone());
                    }
                }
            }
        } else {
            if inside(begin, &edge_vector) {
                let intersection = intersect(&polygon_edge_vector, &edge_vector);
                if let Some(p) = intersection {
                    result.push(p);
                }
            }
        }
    }
    if result.len() > 0 {
        Some(Polygon::new(polygon.color.clone(), result))
    } else {
        None
    }
}

pub fn sutherland_hodgman_polygon_clip(polygon: &Polygon, clipping_polygon: &Vec<Point2d>) -> Option<Polygon> {
    let length = clipping_polygon.len();
    let mut result: Option<Polygon> = Some(polygon.clone());
    for i in 0..length {
        let begin = clipping_polygon.get(i).unwrap();
        let end_index = if i == (length-1) {0} else {i+1};
        let end = clipping_polygon.get(end_index).unwrap();
        match result {
            None => {
                break;
            }
            Some(p) => {
                result = sutherland_hodgman_polygon_edge_clip(&p, (&begin, &end));
            }
        }
    }
    result
}