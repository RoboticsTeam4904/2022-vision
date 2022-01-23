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
    assert!(det.abs() > 1e-4);  // colinear or smt

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

#[cfg(test)]
mod tests {
    use super::*;

    trait Nearby {
        fn near(self, other: &Self, thresh: f64) -> bool;
    }
    impl Nearby for Point {
        fn near(self, other: &Point, thresh: f64) -> bool {
            let delta = self - *other;
            println!("dist: {}", delta.mag());
            return delta.squared_mag() <= thresh.powi(2)
        }
    }

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
