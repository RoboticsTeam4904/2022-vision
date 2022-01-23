use vision_tapes::utility::Point;

/// A translation with an orientation at the end. For example, the position of a vision pattern and
/// its orientation relative to our facing direction. 
/// x = horizontal shift, y = vertical height, z = forwards distance
pub struct Pose {
    pos: Point,
    yaw: f64,
    roll: f64,
    pitch: f64,
}

impl Pose {
    pub fn new(pos: Point, yaw: f64, roll: f64, pitch: f64) -> Pose {
        Pose { pos, yaw, pitch, roll }
    }
    pub fn from_numbers(dist: f64, shift: f64, height: f64,
                        yaw: f64, roll: f64, pitch: f64) -> Self {
        Pose::new(Point::new(shift, height, dist), yaw, roll, pitch)
    }
    pub fn from_orientation(fwd: Point, roll: f64) -> Self {
        Pose::from_numbers(0., 0., 0., fwd.x.atan2(fwd.z), roll, fwd.y.atan2(fwd.z) )
    }
    /// Create a pose from a look vector and an up vector
    pub fn from_orientation_vectors(fwd_vec: Point, up_vec: Point) -> Self {
        unimplemented!("the up orientation is not implemented");
        Pose::from_orientation(fwd_vec, 0. /* TODO @tainish rotate the up vector s.t. fwd points in +z; then do it as a 2d problem */)
    }
    pub fn from_position(pos: Point) -> Self {
        Pose::new(pos, 0., 0., 0.)
    }
    pub fn x(&self)             -> f64 { self.shift }
    pub fn y(&self)             -> f64 { self.height }
    pub fn z(&self)             -> f64 { self.dist }
    pub fn scaled(&self, scalar: f64) -> Pose {
        Pose::new(self.pos * scalar, self.yaw, self.roll, self.pitch)
    }
    // TODO
    //pub fn chained(&self, next: &Pose) -> Pose {
    //    Pose::new(self.yaw + next.yaw, self.roll + next.roll, )
    //}
}
