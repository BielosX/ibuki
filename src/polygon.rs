use std::collections::{HashMap, HashSet};
use std::iter::Map;
use crate::draw::Draw;
use crate::pixel_color::PixelColor;
use crate::point2d::Point2d;
use crate::raw_canvas::RawCanvas;

pub struct Polygon {
    points: Vec<Point2d>,
    color: PixelColor
}

impl Polygon {
    pub fn new(color: PixelColor, points: Vec<Point2d>) -> Polygon {
        Polygon { color, points }
    }
}

#[derive(Copy, Clone)]
struct Fraction {
    nominator: i32,
    denominator: i32
}

#[derive(Copy, Clone)]
struct LowerEndpointInc {
    increment: i32,
    fraction: Fraction
}

#[derive(Copy, Clone)]
struct SegmentDesc {
    y_max: u32,
    lower_endpoint_x: LowerEndpointInc,
    slope_inv: Fraction
}

struct EdgeTable {
    rows: HashMap<u32, Vec<SegmentDesc>>
}

impl EdgeTable {
    fn from_points(points: &Vec<Point2d>) -> EdgeTable {
        let mut rows: HashMap<u32, Vec<SegmentDesc>> = HashMap::new();
        let length = points.len();
        if length >= 2 {
            for x in 0..length {
                let fst_idx = x;
                let mut snd_idx = x + 1;
                if x == length - 1 {
                    snd_idx = 0;
                }
                let first_point = points.get(fst_idx).unwrap();
                let second_point = points.get(snd_idx).unwrap();
                let smaller_x = (if first_point.x < second_point.x { first_point.x } else { second_point.x }).round() as u32;
                let bigger_x = (if first_point.x > second_point.x { first_point.x } else { second_point.x }).round() as u32;
                let smaller_y = (if first_point.y < second_point.y { first_point.y } else { second_point.y }).round() as u32;
                let bigger_y = (if first_point.y > second_point.y { first_point.y } else { second_point.y }).round() as u32;
                let farthest_y = (if first_point.x > second_point.x { first_point.y } else { second_point.y }).round() as u32;
                let nearest_y = (if first_point.x < second_point.x { first_point.y } else { second_point.y }).round() as u32;
                let delta_x = bigger_x as i32 - smaller_x as i32;
                let delta_y = farthest_y as i32 - nearest_y as i32;
                let nominator = if delta_x < 0 && delta_y < 0 {
                    delta_x.abs()
                } else if delta_y < 0 {
                    -delta_x
                } else {
                    delta_x
                };
                let denominator = delta_y.abs();
                let lower_endpoint_x = (if first_point.y < second_point.y { first_point.x } else { second_point.x }).round() as i32;
                let lower_endpoint_inc = LowerEndpointInc { increment: lower_endpoint_x, fraction: Fraction {nominator: 0, denominator} };
                let desc = SegmentDesc { y_max: bigger_y, lower_endpoint_x: lower_endpoint_inc, slope_inv: Fraction {nominator, denominator } };
                let row = rows.entry(smaller_y).or_insert(Vec::new());
                row.push(desc);
            }
        }
        for (_, val) in rows.iter_mut() {
            val.sort_by(|a, b|  a.lower_endpoint_x.increment.cmp(&b.lower_endpoint_x.increment));
        }
        EdgeTable { rows }
    }
}

struct ActiveEdgeTable {
    edges: Vec<SegmentDesc>
}

impl ActiveEdgeTable {
    fn new() -> ActiveEdgeTable {
        ActiveEdgeTable { edges: Vec::new() }
    }

    fn insert_row(&mut self, y: u32, edge_table: &EdgeTable) {
        match edge_table.rows.get(&y) {
            None => {}
            Some(row) => {
                for entry in row.iter() {
                    self.edges.push(entry.clone());
                }
                self.edges.sort_by(|a, b|  a.lower_endpoint_x.increment.cmp(&b.lower_endpoint_x.increment));
            }
        }
    }

    fn remove_lover_edges(&mut self, row: u32) {
        let mut idx_to_remove: HashSet<usize> = HashSet::new();
        let length = self.edges.len();
        for x in 0..length {
            if self.edges.get(x).unwrap().y_max < row {
                idx_to_remove.insert(x);
            }
        }
        let mut new_edges = Vec::new();
        for x in 0..length {
            if !idx_to_remove.contains(&x) {
                new_edges.push(self.edges.get(x).unwrap().clone());
            }
        }
        self.edges = new_edges;
    }

    fn increment_row(&mut self) {
        for entry in self.edges.iter_mut() {
            let mut nominator = entry.lower_endpoint_x.fraction.nominator + entry.slope_inv.nominator;
            let denominator = entry.lower_endpoint_x.fraction.denominator;
            if denominator != 0 {
                entry.lower_endpoint_x.increment += nominator / denominator;
                if nominator.abs() > denominator {
                    let count = (nominator / denominator).abs();
                    if nominator < denominator {
                        nominator += count * denominator;
                    } else {
                        nominator -= count * denominator;
                    }
                }
            }
            entry.lower_endpoint_x.fraction.nominator = nominator;
        }
    }
}

impl Draw for Polygon {
    fn draw(&self, canvas: &RawCanvas) {
        let edge_table = EdgeTable::from_points(&self.points);
        let mut active_edge_table = ActiveEdgeTable::new();
        let smallest_y = self.points.iter().min_by(|a, b| a.y.total_cmp(&b.y)).unwrap().y.round() as u32;
        let biggest_y = self.points.iter().max_by(|a, b| a.y.total_cmp(&b.y)).unwrap().y.round() as u32;
        for row in smallest_y..(biggest_y + 1) {
            active_edge_table.remove_lover_edges(row);
            active_edge_table.insert_row(row, &edge_table);
            let length = active_edge_table.edges.len();
            for x in 0..(length - 1) {
                let first_edge = active_edge_table.edges.get(x).unwrap();
                let second_edge = active_edge_table.edges.get(x + 1).unwrap();
                if x % 2 == 0 {
                    for x_coord in (first_edge.lower_endpoint_x.increment)..(second_edge.lower_endpoint_x.increment) {
                        canvas.put_pixel(x_coord as u32, row, &self.color);
                    }
                }
            }
            active_edge_table.increment_row()
        }
    }
}