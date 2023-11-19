use crate::features::{matrice::Matrice, point::Point};

use super::Shape;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrice,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrice::identity_matrix(4),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrice::identity_matrix(4),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, r: &crate::features::ray::Ray) -> Option<Vec<f64>> {
        let r = r.transform(&self.transform.inverse()?);
        let sphere_to_ray = r.origin - Point::default();
        let a = r.direction.dot_product(&r.direction);
        let b = 2.0 * r.direction.dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some(vec![t1, t2])
    }

    fn set_transform(&mut self, transform: Matrice) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod sphere_intersects_tests {
    use crate::features::{ray::Ray, vector::Vector};

    use super::*;
    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }
    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }
    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs, None);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}

#[cfg(test)]
mod sphere_shape_trait_tests {
    use crate::features::transormations::translation;

    use super::*;
    #[test]
    fn default_transformation() {
        let s = Sphere::default();
        assert_eq!(s.transform, Matrice::identity_matrix(4));
    }

    #[test]
    fn change_sphere_transformation() {
        let mut s = Sphere::default();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t.clone());
        assert_eq!(s.transform, t);
    }
}
