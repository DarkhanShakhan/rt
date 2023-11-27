use super::{ray::Ray, shape::Shape};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Intersection {
    pub shape_id: String,
    pub t: f64,
}

impl Intersection {
    pub fn new(shape_id: &str, t: f64) -> Self {
        Intersection {
            shape_id: shape_id.to_string(),
            t,
        }
    }
    pub fn intersects(s: &dyn Shape, r: &Ray) -> Option<Vec<Self>> {
        if let Some(intersects) = s.intersect(r) {
            let ixs: Vec<Self> = intersects
                .iter()
                .map(|t| Intersection::new(&s.get_shape_id(), *t))
                .collect();
            return Some(ixs);
        }
        None
    }
}

pub fn sort_intersections(xs: &mut [Intersection]) {
    xs.sort_by(|a, b| a.t.total_cmp(&b.t));
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    xs.into_iter().find(|i| i.t > 0.0)
}

#[cfg(test)]
mod intersection_tests {
    use super::*;
    use crate::features::shape::sphere::Sphere;
    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::default();
        let ix = Intersection::new(&sphere.get_shape_id(), 3.5);
        assert_eq!(ix.t, 3.5);
        assert_eq!(sphere.get_shape_id(), ix.shape_id);
    }
}

#[cfg(test)]
mod hit_tests {
    use crate::features::shape::sphere::Sphere;

    use super::*;
    #[test]
    fn all_intersections_have_positive_t() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(&sphere.get_shape_id(), 1.0);
        let i2 = Intersection::new(&sphere.get_shape_id(), 2.0);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        let i = hit(xs).unwrap();
        assert_eq!(i.t, 1.0);
    }
    #[test]
    fn some_intersections_have_negative_t() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(&sphere.get_shape_id(), -1.0);
        let i2 = Intersection::new(&sphere.get_shape_id(), 1.0);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        let i = hit(xs).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn when_all_intersections_have_negative_t() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(&sphere.get_shape_id(), -2.0);
        let i2 = Intersection::new(&sphere.get_shape_id(), -1.0);
        let mut xs = vec![i1, i2];
        sort_intersections(&mut xs);
        assert_eq!(None, hit(xs));
    }

    #[test]
    fn always_the_lowest_nonnegative_intersection() {
        let sphere = Sphere::default();
        let i1 = Intersection::new(&sphere.get_shape_id(), 5.0);
        let i2 = Intersection::new(&sphere.get_shape_id(), 7.0);
        let i3 = Intersection::new(&sphere.get_shape_id(), -3.0);
        let i4 = Intersection::new(&sphere.get_shape_id(), 2.0);
        let mut xs = vec![i1, i2, i3, i4];
        sort_intersections(&mut xs);
        assert_eq!(2.0, hit(xs).unwrap().t);
    }
}

#[cfg(test)]
mod ray_sphere_intersection_tests {
    use crate::features::{
        computation::Computation,
        consts::EPSILON,
        point::Point,
        shape::sphere::Sphere,
        transformations::{scaling, translation},
        vector::Vector,
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
    #[test]
    fn his_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Box::<Sphere>::default();
        shape.set_transform(translation(0.0, 0.0, 1.0));
        let i = Intersection::new(&shape.get_shape_id(), 5.0);
        let comps = Computation::new(&r, &i, shape.as_ref());
        assert!(comps.over_point.position.z < (-EPSILON / 2.0));
        assert!(comps.point.position.z > comps.over_point.position.z);
    }
}
