use vision_tapes::utility::Point;

/// x = horizontal shift, y = vertical height, z = forwards distance
pub struct Pose {
    dist: f64,
    shift: f64, 
    height: f64,
    yaw: f64,
    roll: f64,
    pitch: f64,
}

impl Pose {
    pub fn new(dist: f64, shift: f64, height: f64, yaw: f64, roll: f64, pitch: f64) -> Self {
        Pose { dist, shift, height, yaw, roll, pitch }
    }
    pub fn from_orientation(fwd: Point, roll: f64) -> Self {
        Pose::new(0., 0., 0., fwd.x.atan2(fwd.z), roll, fwd.y.atan2(fwd.z) )
    }
    /// Create a pose from a look vector and an up vector
    pub fn from_orientation_vectors(fwd_vec: Point, up_vec: Point) -> Self {
        unimplemented!("the up orientation is not implemented");
        Pose::from_orientation(fwd_vec, 0. /* TODO @tainish rotate the up vector s.t. fwd points in +z; then do it as a 2d problem */)
    }
    pub fn from_position(pos: Point) -> Self {
        Pose::new(pos.z, pos.x, pos.y, 0., 0., 0.)
    }
    pub fn x(&self) -> f64 { self.shift }
    pub fn y(&self) -> f64 { self.height }
    pub fn z(&self) -> f64 { self.dist }
    pub fn pos(&self) -> Point { 
        Point::new(self.x(), self.y(), self.z())
    }
    pub fn norm(&self) -> f64 { self.pos().squared_mag() }
    pub fn squared_mag(&self) -> f64 { self.norm() }
}
