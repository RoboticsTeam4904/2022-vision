extern crate nalgebra as na;
use na::{Point3, Translation3};
// use std::f64::consts::PI;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn squared_mag(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn debug(self, msg: &str) -> Self {
        println!("{} {:?}", msg, self);
        self
    }

    pub fn mag(&self) -> f64 {
        self.squared_mag().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self * (self.mag().recip())
    }

    pub fn translate(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn update(&mut self, other: Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    pub fn scale(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }

    pub fn volume(&self) -> f64 {
        self.x * self.y * self.z
    }
    pub fn with_x(&self, x: f64) -> Self {
        Self {
            x,
            y: self.y,
            z: self.z,
        }
    }
    pub fn with_y(&self, y: f64) -> Self {
        Self {
            x: self.x,
            y,
            z: self.z,
        }
    }
    pub fn with_z(&self, z: f64) -> Point {
        Self {
            x: self.x,
            y: self.y,
            z,
        }
    }

    pub fn na_point(&self) -> Point3<f32> {
        Point3::<f32>::new(self.x as f32, self.y as f32, self.z as f32)
    }
    pub fn translation(&self) -> Translation3<f32> {
        Translation3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn rotate(&mut self, axis: &Self, theta: f64) {
        self.update(self.rotated(axis, theta))
    }
    pub fn rotated(&self, axis: &Self, theta: f64) -> Self {
        assert!(
            axis.squared_mag() != 0.,
            "The axis of rotation is the zero vector."
        );
        let unit_axis = *axis * axis.mag().recip();
        let vector = self.clone();
        vector * theta.cos()
            + (unit_axis * vector) * theta.sin()
            + unit_axis * (unit_axis.dot(vector)) * (1. - theta.cos())
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Into<(u64, u64, u64)> for Point {
    fn into(self) -> (u64, u64, u64) {
        (self.x as u64, self.y as u64, self.z as u64)
    }
}

impl Into<[f64; 3]> for Point {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(other: (f64, f64, f64)) -> Self {
        Point::new(other.0, other.1, other.2)
    }
}

impl From<(i32, i32, i32)> for Point {
    fn from(other: (i32, i32, i32)) -> Self {
        Point::new(other.0 as f64, other.1 as f64, other.2 as f64)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


