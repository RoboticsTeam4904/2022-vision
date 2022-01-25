use vision_tapes::utility::Point;
use std::f64::consts::PI;
extern crate nalgebra as na;
use na::{ Vector3, Rotation3 };

/// A translation with an orientation at the end. For example, the position of a vision pattern and
/// its orientation relative to our facing direction. 
/// 
/// Rotations are calcualted in the order yaw, then pitch, then roll
/// x = horizontal shift, z = vertical height, y = forwards distance
#[derive(Debug)]
pub struct Pose {
    pub pos: Point,
    yaw: f64,
    roll: f64,
    pitch: f64,
}

impl Pose {
    pub fn debug(self, msg: &str) -> Self {
        println!("{} {:?}", msg, self);
        self
    }
    // constructors
    pub fn new(pos: Point, yaw: f64, roll: f64, pitch: f64) -> Pose {
        Pose { pos, yaw, pitch, roll }
    }
    pub fn from_numbers(dist: f64, shift: f64, height: f64,
                        yaw: f64, roll: f64, pitch: f64) -> Self {
        Self::new(Point::new(shift, dist, height), yaw, roll, pitch)
    }
    pub fn from_orientation(look: Point, roll: f64) -> Self {
        Self::from_numbers(0., 0., 0., look.x.atan2(look.y), roll, look.z.atan2(look.x.hypot(look.y)) )
    }
    /// Create a pose from a look vector and an up vector
    pub fn from_orientation_vectors(look: Point, up: Point) -> Self {
        let pose = Self::from_orientation(look, 0.);
        let up_vec = up
            .rotated(&Point::new(-look.y, look.x, 0.), pose.pitch)
            .rotated(&Point::new(0., 0., 1.), pose.yaw);
        pose.with_roll(up_vec.x.atan2(up_vec.z))
    }
    pub fn from_pos(pos: Point) -> Self {
        Self::new(pos, 0., 0., 0.)
    }

    // getters
    /// get the horizontal (x-direction) offset of the object
    pub fn shift(&self)                 -> f64 { self.pos.x }
    /// get the vertical (z-direction) offset of the object
    pub fn height(&self)                -> f64 { self.pos.z }
    /// get the forwards (y-direction) offset of the object
    pub fn dist(&self)                  -> f64 { self.pos.y }
    /// get a copy of the object with a different position, but without changing the orientation
    pub fn up(&self) -> Point {
        let perp = Point::new(self.yaw.cos(), -self.yaw.sin(), 0.);
        let up = Point::new(0., 0., 1.).rotated(&perp, self.pitch);
        up.rotated(&(up * perp), self.roll).normalize()
    }
    pub fn look(&self) -> Point {
        Point::new(self.yaw.sin()*self.pitch.cos(), self.yaw.cos()*self.pitch.cos(), self.pitch.sin()).normalize()
    }

    // transforms
    pub fn with_pos(&self, pos: Point) -> Self { Self::new(pos, self.yaw, self.roll, self.pitch) }
    /// get a copy of the object with a different shift, but without changing the orientation
    pub fn with_shift(&self, x: f64) -> Self { self.with_pos(self.pos.with_x(x)) }
    /// get a copy of the object with a different height, but without changing the orientation
    pub fn with_dist(&self, y: f64) -> Self { self.with_pos(self.pos.with_y(y)) }
    /// get a copy of the object with a different distance, but without changing the orientation
    pub fn with_height(&self, z: f64) -> Self { self.with_pos(self.pos.with_z(z)) }

    /// get a copy of the object with a different yaw, but without changing the position
    pub fn with_yaw(&self, theta: f64) -> Self { Pose::new(self.pos, theta, self.roll, self.pitch) }
    /// get a copy of the object with a different roll, but without changing the position
    pub fn with_roll(&self, theta: f64) -> Self { Pose::new(self.pos, self.yaw, theta, self.pitch) }
    /// get a copy of the object with a different pitch, but without changing the position
    pub fn with_pitch(&self, theta: f64) -> Self { Pose::new(self.pos, self.yaw, self.roll, theta) }

    /// check if two poses are within epsilon of each other (both delta pos and each orientation)
    pub fn like(&self, other: &Self, epsilon: f64) -> bool {
        println!("yrp1 {:.3} {:.3} {:.3} yrp2 {:.3} {:.3} {:.3}", self.yaw, self.roll, self.pitch, other.yaw, other.roll, other.pitch);
        (self.pos - other.pos).mag() <= epsilon &&
        (self.yaw - other.yaw).abs() <= epsilon && 
        (self.roll - other.roll).abs() <= epsilon &&
        (self.pitch - other.pitch).abs() <= epsilon
    }

    /// get a copy of the object `scalar` times further away
    pub fn scaled(&self, scalar: f64) -> Self {
        Self::new(self.pos * scalar, self.yaw, self.roll, self.pitch)
    }
    /// get the sum pose of another pose attached to the face of this one
    pub fn chain(&self, next: &Pose) -> Pose {
        let rot = Rotation3::face_towards(
            &Vector3::new(self.look().x, self.look().z, self.look().y),
            &Vector3::new(self.up().x,   self.up().z,   self.up().y)
        );
        //println!("rotation {:?}", rot * Vector3::new(0., 0., 1.));
        println!("rotation {:?}", rot * Vector3::z());
        let pos = Vector3::new(next.pos.x, next.pos.z, next.pos.y);
        let pos = rot * pos;
        let pos = Point::new(pos.x, pos.z, pos.y) + self.pos;
        println!("wheeeeeeeee {:?}, yrp {} {} {}", pos, self.yaw + next.yaw, self.roll + next.roll, self.pitch + next.pitch);
        Self::new(pos, self.yaw + next.yaw, self.roll + next.roll, self.pitch + next.pitch)
    }
}

