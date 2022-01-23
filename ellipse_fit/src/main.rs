use vision_tapes::utility::Point;
use stdvis_core::{ traits::ImageData, types::Image };

mod utility;
use utility::Pose;

fn fit_flat_circle(a: Point, b: Point, c: Point) -> Point {
    // algorithm: https://stackoverflow.com/a/4103418/10372825
    assert_eq!(b.z, a.z);
    assert_eq!(b.z, c.z);

    let offset = b.squared_mag();
    let bc = (a.x.powi(2) + a.y.powi(2) - offset) / 2.;
    let cd = (offset - c.x.powi(2) - c.y.powi(2)) / 2.;
    let det = (a.x - b.x) * (b.y - c.y) - (b.x - c.x) * (a.y - b.y);

    println!("determinant: {det}");
    assert!(det.abs() > f64::EPSILON);  // colinear or smt

    let cx = (bc * (b.y - c.y) - cd * (a.y - b.y)) / det;
    let cy = (cd * (a.x - b.x) - bc * (b.x - c.x)) / det;
    Point::new(cx, cy, b.z)
}

fn main() {
    let a = Point::new(-8.8,  0.9, 0.);
    let b = Point::new( 2.5, -2.7, 0.);
    let c = Point::new( 8.3,  6.5, 0.);
    let p = fit_flat_circle(a, b, c);
    println!("Hello, {p:?}");
}

pub trait Nearby {
    fn near(self, other: &Self, thresh: f64) -> bool;
}
impl Nearby for Point {
    fn near(self, other: &Point, thresh: f64) -> bool {
        let delta = self - *other;
        println!("dist: {}", delta.mag());
        return delta.squared_mag() <= thresh.powi(2)
    }
}

#[cfg(test)]
mod test_circle_fit {
    use super::*;

    #[test]
    fn circle_fit_1() {
        let a = Point::new(-8.8,  0.9, 0.);
        let b = Point::new( 2.5, -2.7, 0.);
        let c = Point::new( 8.3,  6.5, 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(-1., 5.9, 0.), 1e-1));
    }
    #[test]
    fn circle_fit_2() {
        let a = Point::new(-30.,  0.9, 0.);
        let b = Point::new(-19.,  33., 0.);
        let c = Point::new( 8.3,  6.5, 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(-12.2, 12.7, 0.), 1e-1));
    }
    #[test]
    fn circle_fit_3() {
        let a = Point::new(-30.,  0.9, 0.);
        let b = Point::new(-19.,  33., 0.);
        let c = Point::new(-21.,  16., 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(-60.4, 29.3, 0.), 1e-1));
    }
    #[test]
    fn circle_fit_4() {
        let a = Point::new(-30.,  0.9, 0.);
        let b = Point::new(-19.,  33., 0.);
        let c = Point::new(-31.2, -2.74, 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(1503.8, -506.8, 0.), 1e-1));
    }
}

#[cfg(test)]
mod test_pose_rotation {
    use super::*;
    use std::f64::consts::{ PI, FRAC_PI_4, FRAC_PI_2, FRAC_PI_6 };

    #[test]
    fn up_bijection_1() {
        let look = Point::new(0., 1., 0.);
        let up = Point::new(1., 0., 1.).normalize();
        assert!(up.near(&Pose::from_orientation_vectors(look, up).up(), 1e-6))
    }
    #[test]
    fn up_bijection_2() {
        let look = Point::new(1., 1., 0.);
        let up = Point::new(1., -1., 0.).normalize();
        assert!(up.near(&Pose::from_orientation_vectors(look, up).up(), 1e-6))
    }
    #[test]
    fn up_bijection_3() {
        let look = Point::new(1., 1., 0.);
        let up = Point::new(1., -1., 1.).normalize();
        assert!(up.near(&Pose::from_orientation_vectors(look, up).up(), 1e-6))
    }
    #[test]
    fn up_bijection_4() {
        let look = Point::new(1., 1., 1.);
        let up = Point::new(-1., 2., -1.).normalize();
        assert!(up.near(&Pose::from_orientation_vectors(look, up).up(), 1e-6))
    }
    #[test]
    fn up_bijection_5() {
        let look = Point::new(1., 1., 1.);
        let up = Point::new(-1., 1., 0.).normalize();
        assert!(up.near(&Pose::from_orientation_vectors(look, up).up(), 1e-6))
    }


    #[test]
    /// no roll
    fn orientation_vec_constructor_1() {
        let look = Point::new(0., 1., 0.);
        let pose1 = Pose::from_orientation(look, 0.);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(0., 0., 1.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    /// 180 roll
    fn orientation_vec_constructor_2() {
        let look = Point::new(1., 1., 1.);
        let pose1 = Pose::from_orientation(look, PI);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(1., 1., -2.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    /// 90 roll
    fn orientation_vec_constructor_3() {
        let look = Point::new(-1., -1., 2.);
        let pose1 = Pose::from_orientation(look, FRAC_PI_2);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(0., 1., -1.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    /// whack
    fn orientation_vec_constructor_4() {
        let look = Point::new(1., 1., 1.);
        let pose1 = Pose::from_orientation(look, 0.);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(-1., -1., 2.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    /// whack
    fn orientation_vec_constructor_5() {
        let look = Point::new(1., 1., 1.);
        let pose1 = Pose::from_orientation(look, -5. * FRAC_PI_6);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(0., 1., -1.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
}
