use std::rc::Rc;

use super::{
    intersection::{self, Intersection},
    point::{self, Point},
    ray::Ray,
    shape::Shape,
    vector::Vector,
};

pub struct Computation {
    pub t: f64,
    pub object_id: String,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl Computation {
    pub fn new(ray: &Ray, i: &Intersection, s: &Box<dyn Shape>) -> Self {
        let point = ray.position(i.t);
        let eyev = -ray.direction;
        let mut normalv = s.normal_at(point).unwrap_or_default();
        let inside: bool;
        if normalv.dot_product(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }
        Computation {
            t: i.t,
            object_id: i.shape_id.clone(),
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod computation_tests {
    use crate::features::shape::sphere::Sphere;

    use super::*;
    #[test]
    fn precompute_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::default());
        let i = Intersection::new(shape.get_shape_id(), 4.0);
        let comps = Computation::new(&r, &i, &shape);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object_id, i.shape_id);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn intersection_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::default());
        let i = Intersection::new(shape.get_shape_id(), 1.0);
        let comps = Computation::new(&r, &i, &shape);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn intersection_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::default());
        let i = Intersection::new(shape.get_shape_id(), 1.0);
        let comps = Computation::new(&r, &i, &shape);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    }
}
