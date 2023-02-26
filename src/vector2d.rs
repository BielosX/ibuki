use std::ops;
use std::ops::{Add, Div, Mul};
use crate::point2d::Point2d;

pub struct Vector2d {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32
}

impl Vector2d {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Vector2d {
        Vector2d { x0, y0, x1, y1 }
    }

    pub fn from_2d_points(from: &Point2d, to: &Point2d) -> Vector2d {
        Vector2d::new(from.x, from.y, to.x, to.y)
    }

    pub fn length(&self) -> f32 {
        ((self.x1 - self.x0).powi(2) + (self.y1 - self.y0).powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Vector2d) -> f32 {
        (self.x1 - self.x0) * (other.x1 - other.x0) + (self.y1 - self.y0) * (other.y1 - other.y0)
    }

    pub fn normal_left(&self) -> Vector2d {
        let len = self.length();
        Vector2d::new(0.0, 0.0, -(self.y1 - self.y0), self.x1 - self.x0) / len
    }

    pub fn normal_right(&self) -> Vector2d {
        let len = self.length();
        Vector2d::new(0.0, 0.0, self.y1 - self.y0, self.x1 - self.x0) / len
    }

    pub fn get_from(&self) -> Point2d {
        Point2d::new(self.x0, self.y0)
    }

    pub fn get_to(&self) -> Point2d {
        Point2d::new(self.x1, self.y1)
    }
}

impl Div<f32> for Vector2d {
    type Output = Vector2d;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2d::new(self.x0 / rhs, self.y0 / rhs, self.x1 / rhs, self.y1 / rhs)
    }
}

impl Mul<f32> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2d::new(self.x0 * rhs, self.x0 * rhs, self.x1 * rhs, self.y1 * rhs)
    }
}

impl<'a> Mul<f32> for &'a Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2d::new(self.x0 * rhs, self.x0 * rhs, self.x1 * rhs, self.y1 * rhs)
    }
}

impl Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Self::Output {
        Vector2d::new(self.x0 + rhs.x0, self.y0 + rhs.y0, self.x1 + rhs.x1, self.y1 + rhs.y1)
    }
}

impl<'a, 'b> Add<&'a Vector2d> for &'b Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: &'a Vector2d) -> Self::Output {
        Vector2d::new(self.x0 + rhs.x0, self.y0 + rhs.y0, self.x1 + rhs.x1, self.y1 + rhs.y1)
    }
}