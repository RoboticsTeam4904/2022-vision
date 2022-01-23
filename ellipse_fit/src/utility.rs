use vision_tapes::utility::Point;
use std::f64::consts::PI;

/// A translation with an orientation at the end. For example, the position of a vision pattern and
/// its orientation relative to our facing direction. 
/// 
/// Rotations are calcualted in the order yaw, then pitch, then roll
/// x = horizontal shift, y = vertical height, z = forwards distance
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
        Self::new(Point::new(shift, height, dist), yaw, roll, pitch)
    }
    pub fn from_orientation(look: Point, roll: f64) -> Self {
        Self::from_numbers(0., 0., 0., look.x.atan2(look.z), roll, look.y.atan2(look.x.hypot(look.z)) )
    }
    /// Create a pose from a look vector and an up vector
    pub fn from_orientation_vectors(look: Point, up: Point) -> Self {
        let pose = Self::from_orientation(look, 0.);
        let up_vec = up
            .rotated(&Point::new(-look.z, 0., look.x), pose.pitch).debug("construct 1")
            .rotated(&Point::new(0., 1., 0.), -pose.yaw).debug("construct 2");
        println!("roll angle: {}", up_vec.x.atan2(up_vec.y) / PI);
        pose.with_roll(up_vec.x.atan2(up_vec.y))
    }
    pub fn up(&self) -> Point {
        let perp = Point::new(self.yaw.cos(), 0., self.yaw.sin()).debug("up/perp");
        let up = Point::new(0., 1., 0.).rotated(&perp, self.pitch).debug("up/up");
        up.rotated(&(perp * up).debug("up cross perp"), self.roll).normalize()
    }
    pub fn from_position(pos: Point) -> Self {
        Self::new(pos, 0., 0., 0.)
    }

    // getters
    /// get the horizontal (x-direction) offset of the object
    pub fn shift(&self)                 -> f64 { self.pos.x }
    /// get the vertical (y-direction) offset of the object
    pub fn height(&self)                -> f64 { self.pos.y }
    /// get the forwards (z-direction) offset of the object
    pub fn dist(&self)                  -> f64 { self.pos.z }
    /// get a copy of the object with a different position, but without changing the orientation

    // transforms
    pub fn with_pos(&self, pos: Point) -> Self { Self::new(pos, self.yaw, self.roll, self.pitch) }
    /// get a copy of the object with a different shift, but without changing the orientation
    pub fn with_shift(&self, x: f64) -> Self { self.with_pos(self.pos.with_x(x)) }
    /// get a copy of the object with a different height, but without changing the orientation
    pub fn with_height(&self, y: f64) -> Self { self.with_pos(self.pos.with_y(y)) }
    /// get a copy of the object with a different distance, but without changing the orientation
    pub fn with_dist(&self, z: f64) -> Self { self.with_pos(self.pos.with_z(z)) }

    /// get a copy of the object with a different yaw, but without changing the position
    pub fn with_yaw(&self, theta: f64) -> Self { Pose::new(self.pos, theta, self.roll, self.pitch) }
    /// get a copy of the object with a different roll, but without changing the position
    pub fn with_roll(&self, theta: f64) -> Self { Pose::new(self.pos, self.yaw, theta, self.pitch) }
    /// get a copy of the object with a different pitch, but without changing the position
    pub fn with_pitch(&self, theta: f64) -> Self { Pose::new(self.pos, self.yaw, self.roll, theta) }

    /// get a copy of the object `scalar` times further away
    pub fn scaled(&self, scalar: f64) -> Self {
        Self::new(self.pos * scalar, self.yaw, self.roll, self.pitch)
    }
    // TODO
    ///// get the sum pose of another pose attached to the face of this one
    //pub fn chained(&self, next: &Pose) -> Pose {
    //    Self::new(self.yaw + next.yaw, self.roll + next.roll, self.pitch + next.pitch)
    //}
}
