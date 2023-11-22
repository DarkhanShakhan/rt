use std::rc::Rc;

use super::{
    intersection::{self, Intersection},
    point::{self, Point},
    ray::Ray,
    shape::Shape,
    vector::Vector,
};

pub struct Computation<S: Shape> {
    pub t: f64,
    pub object: Rc<S>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
}

impl<S: Shape> Computation<S> {
    pub fn new(ray: &Ray, i: &Intersection<S>) -> Self {
        let point = ray.position(i.t);
        let eyev = -ray.direction;
        let normalv = i.shape.normal_at(point).unwrap_or_default();
        Computation {
            t: i.t,
            object: i.shape.clone(),
            point,
            eyev,
            normalv,
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
        let shape = Sphere::default();
        let i = Intersection::new(Rc::new(shape), 4.0);
        let comps = Computation::new(&r, &i);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.shape);
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
}
