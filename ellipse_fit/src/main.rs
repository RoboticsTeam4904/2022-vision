use opencv::core::Point_;
use opencv::types::VectorOfPoint;
use stdvis_core::{traits::ImageData, types::Image};
use vision_tapes::utility::Point;
use nalgebra as na;
use nalgebra::dvector;
use nalgebra::matrix;

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
    assert!(det.abs() > f64::EPSILON); // colinear or smt

    let cx = (bc * (b.y - c.y) - cd * (a.y - b.y)) / det;
    let cy = (cd * (a.x - b.x) - bc * (b.x - c.x)) / det;
    Point::new(cx, cy, b.z)
}

fn fit_ellipse() {
    // let points = vertices.iter().map(|v| {
    //     v.iter().fold(
    //         Point_::<i32>::new(0, 0),
    //         |a, b| if (a.y > b.y) { a } else { b },
    //     )
    // });
    let theta = dvector![0,0,0,0,0,0].transpose();
    let points = vec![Point_::<i32>::new(2, 1), Point_::<i32>::new(0, 2), Point_::<i32>::new(1, 1)]; 
    let mut M = na::Matrix6::from_element(0i32);    
    for p in points {
	let u = dvector![p.x*p.x, p.x*p.y, p.y*p.y, p.x, p.y, 1].transpose();	
	M += theta.clone().transpose() *
	    u.clone() * u.transpose() *
	    theta.clone(); // TODO find a way around this .clone()
    }
    let tmp: na::Matrix2<i32> = matrix![1,0; 0,0];
    let tmp2: na::Matrix3<i32> = matrix![0,0,2; 0,-1,0; 2,0,0];
    let F = tmp.kronecker(&tmp2);
    // dbg!(F);
    // dbg!(theta.transpose() * M);
    // let loss = (theta.transpose() * M * theta)/(theta.transpose() * F * theta);
    // dbg!(loss);
}

fn main() {
    fit_ellipse();
    // let a = Point::new(-8.8, 0.9, 0.);
    // let b = Point::new(2.5, -2.7, 0.);
    // let c = Point::new(8.3, 6.5, 0.);
    // let p = fit_flat_circle(a, b, c);
    // println!("Hello, {p:?}");
}

pub trait Nearby {
    fn near(self, other: &Self, thresh: f64) -> bool;
}
impl Nearby for Point {
    fn near(self, other: &Point, thresh: f64) -> bool {
        let delta = self - *other;
        println!("dist: {}", delta.mag());
        return delta.squared_mag() <= thresh.powi(2);
    }
}

#[cfg(test)]
mod test_circle_fit {
    use super::*;

    #[test]
    fn circle_fit_1() {
        let a = Point::new(-8.8, 0.9, 0.);
        let b = Point::new(2.5, -2.7, 0.);
        let c = Point::new(8.3, 6.5, 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(-1., 5.9, 0.), 1e-1));
    }
    #[test]
    fn circle_fit_2() {
        let a = Point::new(-30., 0.9, 0.);
        let b = Point::new(-19., 33., 0.);
        let c = Point::new(8.3, 6.5, 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(-12.2, 12.7, 0.), 1e-1));
    }
    #[test]
    fn circle_fit_3() {
        let a = Point::new(-30., 0.9, 0.);
        let b = Point::new(-19., 33., 0.);
        let c = Point::new(-21., 16., 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(-60.4, 29.3, 0.), 1e-1));
    }
    #[test]
    fn circle_fit_4() {
        let a = Point::new(-30., 0.9, 0.);
        let b = Point::new(-19., 33., 0.);
        let c = Point::new(-31.2, -2.74, 0.);
        assert!(fit_flat_circle(a, b, c).near(&Point::new(1503.8, -506.8, 0.), 1e-1));
    }
}

#[cfg(test)]
mod test_pose_rotation {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};

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
    fn look_bijection_up() {
        let look = Point::new(0., 0., 1.).normalize();
        assert!(look.near(&Pose::from_orientation(look, 1.2).look(), 1e-6));
    }
    #[test]
    fn look_bijection_fwd() {
        let look = Point::new(0., 1., 0.).normalize();
        assert!(look.near(&Pose::from_orientation(look, 1.2).look(), 1e-6));
    }
    #[test]
    fn look_bijection_left() {
        let look = Point::new(-1., 0., 0.).normalize();
        assert!(look.near(&Pose::from_orientation(look, 1.2).look(), 1e-6));
    }
    #[test]
    fn look_bijection_1() {
        let look = Point::new(1., 1., 1.).normalize();
        assert!(look.near(&Pose::from_orientation(look, 1.2).look(), 1e-6));
    }
    #[test]
    fn look_bijection_2() {
        let look = Point::new(1., -1., 1.).normalize();
        assert!(look.near(&Pose::from_orientation(look, 1.2).look(), 1e-6));
    }
    #[test]
    fn look_bijection_3() {
        let look = Point::new(-200., 1.4934, 23.2399).normalize();
        assert!(look.near(&Pose::from_orientation(look, 1.2).look(), 1e-6));
    }

    #[test]
    /// test scaling
    fn scaled() {
        assert!(Pose::from_pos(Point::new(1., 1., 2.))
            .scaled(-3.)
            .pos
            .near(&Point::new(-3., -3., -6.), 1e-6));
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
    fn orientation_vec_constructor_4() {
        let look = Point::new(1., 1., 1.);
        let pose1 = Pose::from_orientation(look, 0.);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(-1., -1., 2.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    fn orientation_vec_constructor_5() {
        let look = Point::new(1., 1., 1.);
        let pose1 = Pose::from_orientation(look, -5. * FRAC_PI_6);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(0., 1., -1.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    fn orientation_vec_constructor_nonortho_up() {
        let look = Point::new(1., 1., 1.);
        let pose1 = Pose::from_orientation(look, 0.);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(0., 0., 1.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }
    #[test]
    fn orientation_vec_constructor_nonortho_down() {
        let look = Point::new(10., -8., 1.);
        let pose1 = Pose::from_orientation(look, PI);
        let pose2 = Pose::from_orientation_vectors(look, Point::new(0., 0., -1.));
        assert!(pose1.up().near(&pose2.up(), 1e-3));
    }

    #[test]
    fn chain_poses_orientation_identity_left() {
        let pose1 = Pose::from_orientation(Point::new(0., 1., 0.), 0.);
        let pose2 = Pose::from_orientation(Point::new(1., 1., 1.), 0.);
        assert!(pose1
            .chain(&pose2)
            .like(&Pose::from_orientation(Point::new(1., 1., 1.), 0.), 1e-6));
    }
    #[test]
    fn chain_poses_orientation_identity_right() {
        let pose1 = Pose::from_orientation(Point::new(1., 1., 1.), 0.);
        let pose2 = Pose::from_orientation(Point::new(0., 1., 0.), 0.);
        assert!(pose1
            .chain(&pose2)
            .like(&Pose::from_orientation(Point::new(1., 1., 1.), 0.), 1e-6));
    }
    #[test]
    fn chain_poses_orientation_identity_roll() {
        let pose1 = Pose::from_orientation(Point::new(1., 1., 1.), 1.);
        let pose2 = Pose::from_orientation(Point::new(0., 1., 0.), 2.);
        assert!(pose1
            .chain(&pose2)
            .like(&Pose::from_orientation(Point::new(1., 1., 1.), 3.), 1e-6));
    }

    #[test]
    fn chain_poses_orientation_identity_offsets_1() {
        let pose1 = Pose::from_orientation(Point::new(1., 0., 0.), 0.);
        let pose2 = Pose::from_orientation(Point::new(1., 1., 1.), 0.).with_dist(1.);
        assert!(pose1.chain(&pose2).like(
            &Pose::from_orientation(Point::new(1., -1., 1.), 0.).with_shift(1.),
            1e-6
        ));
    }
    #[test]
    fn chain_poses_orientation_identity_offsets_2() {
        let pose1 = Pose::from_orientation(Point::new(1., 1., 0.), 0.);
        let pose2 = Pose::from_orientation(Point::new(0., 0., 1.), 0.).with_dist(2f64.sqrt());
        assert!(pose1.chain(&pose2).like(
            &Pose::from_orientation(Point::new(0., 0., 1.), -FRAC_PI_4)
                .with_shift(1.)
                .with_dist(1.),
            1e-6
        ));
    }
    #[test]
    fn chain_poses_orientation_identity_offsets_3() {
        let pose1 = Pose::from_orientation(Point::new(0., 1., 0.), PI);
        let pose2 = Pose::from_orientation(Point::new(0., 1., 0.), 0.).with_shift(1.);
        assert!(pose1.chain(&pose2).like(
            &Pose::from_orientation(Point::new(0., 1., 0.), PI).with_shift(-1.),
            1e-6
        ));
    }
    #[test]
    fn chain_poses_orientation_identity_offsets_4() {
        let pose1 = Pose::from_orientation(Point::new(1., 1., 0.), PI);
        let pose2 = Pose::from_orientation(Point::new(0., 0., 1.), 0.).with_dist(2f64.sqrt());
        assert!(pose1.chain(&pose2).like(
            &Pose::from_orientation(Point::new(0., 0., -1.), -FRAC_PI_4)
                .with_shift(1.)
                .with_dist(1.),
            1e-6
        ));
    }
}
