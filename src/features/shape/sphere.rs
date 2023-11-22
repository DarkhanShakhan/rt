use crate::features::{material::Material, matrice::Matrice, point::Point, vector::Vector};

use super::Shape;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrice,
    material: Material,
}

impl Sphere {
    pub fn new(transform: Matrice, material: Material) -> Self {
        Sphere {
            transform,
            material,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Matrice::identity_matrix(4), Material::default())
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
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    fn get_material(&self) -> Material {
        self.material.clone()
    }
    fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }
    fn normal_at(&self, world_point: Point) -> Option<Vector> {
        let object_point = self.transform.inverse()? * world_point;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = self.transform.inverse()?.transpose() * object_normal;
        Some(world_normal.normalize())
    }
}

#[cfg(test)]
mod sphere_intersects_tests {
    use crate::features::{ray::Ray, vector::Vector};

    use super::*;
    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }
    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }
    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs, None);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
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

    #[test]
    fn default_material() {
        let s = Sphere::default();
        assert_eq!(s.material, Material::default())
    }
    #[test]
    fn material_assigned() {
        let mut s = Sphere::default();
        let m = Material {
            ambient: 1.0,
            ..Default::default()
        };
        s.material = m.clone();
        assert_eq!(s.material, m);
    }
}

#[cfg(test)]
mod normals_tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use crate::features::transormations::{rotation_z, scaling, translation};

    use super::*;

    #[test]
    fn on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0)).unwrap();
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0))
    }

    #[test]
    fn z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0)).unwrap();
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0))
    }

    #[test]
    fn on_nonaxial() {
        let s = Sphere::default();
        let n = s
            .normal_at(Point::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(
            n,
            Vector::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            )
        )
    }
    #[test]
    fn is_normalized() {
        let s = Sphere::default();
        let n = s
            .normal_at(Point::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn translated_sphere() {
        let mut s = Sphere::default();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s
            .normal_at(Point::new(0.0, 1.70711, -FRAC_1_SQRT_2))
            .unwrap();
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn transformed_sphere() {
        let mut s = Sphere::default();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s
            .normal_at(Point::new(
                0.0,
                2.0_f64.sqrt() / 2.0,
                -(2.0_f64.sqrt() / 2.0),
            ))
            .unwrap();
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
