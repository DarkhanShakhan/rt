use std::rc::Rc;

use super::{ray::Ray, shape::Shape};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Intersection<S: Shape> {
    pub shape: Rc<S>,
    pub t: f64,
}

impl<S: Shape> Intersection<S> {
    pub fn new(shape: Rc<S>, t: f64) -> Self {
        Intersection { shape, t }
    }
    pub fn intersects(s: Rc<S>, r: &Ray) -> Option<Vec<Self>> {
        if let Some(intersects) = s.intersect(r) {
            let ixs: Vec<Self> = intersects
                .iter()
                .map(|t| Intersection::new(s.clone(), *t))
                .collect();
            return Some(ixs);
        }
        None
    }
}

pub fn sort_intersections<S: Shape>(xs: &mut [Intersection<S>]) {
    xs.sort_by(|a, b| a.t.total_cmp(&b.t));
}

pub fn hit<S: Shape>(xs: Vec<Intersection<S>>) -> Option<Intersection<S>> {
    xs.into_iter().find(|i| i.t > 0.0)
}

#[cfg(test)]
mod intersection_tests {
    use super::*;
    use crate::features::{point::Point, shape::sphere::Sphere, vector::Vector};
    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Rc::new(Sphere::new());
        let ix = Intersection::new(sphere.clone(), 3.5);
        assert_eq!(ix.t, 3.5);
        assert_eq!(sphere, ix.shape);
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::features::shape::sphere::Sphere;

    use super::*;
    #[test]
    fn all_intersections_have_positive_t() {
        let sphere = Rc::new(Sphere::new());
        let i1 = Intersection::new(sphere.clone(), 1.0);
        let i2 = Intersection::new(sphere.clone(), 2.0);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        let i = hit(xs).unwrap();
        assert_eq!(i.t, 1.0);
    }
    #[test]
    fn some_intersections_have_negative_t() {
        let sphere = Rc::new(Sphere::new());
        let i1 = Intersection::new(sphere.clone(), -1.0);
        let i2 = Intersection::new(sphere.clone(), 1.0);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        let i = hit(xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn when_all_intersections_have_negative_t() {
        let sphere = Rc::new(Sphere::new());
        let i1 = Intersection::new(sphere.clone(), -2.0);
        let i2 = Intersection::new(sphere.clone(), -1.0);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        assert_eq!(None, hit(xs));
    }

    #[test]
    fn always_the_lowest_nonnegative_intersection() {
        let sphere = Rc::new(Sphere::new());
        let i1 = Intersection::new(sphere.clone(), 5.0);
        let i2 = Intersection::new(sphere.clone(), 7.0);
        let i3 = Intersection::new(sphere.clone(), -3.0);
        let i4 = Intersection::new(sphere.clone(), 2.0);
        let mut xs = vec![i1, i2, i3, i4];
        sort_intersections(&mut xs);
        assert_eq!(2.0, hit(xs).unwrap().t);
    }
}

#[cfg(test)]
mod ray_sphere_intersection_tests {
    use crate::features::{
        point::Point, shape::sphere::Sphere, transormations::scaling, vector::Vector,
    };

    use super::*;
    #[test]
    fn scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_ne!(None, xs);
        let xs = xs.unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 3.0);
        assert_eq!(xs[1], 7.0);
    }

    #[test]
    fn translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.set_transform(scaling(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(None, xs)
    }
}
