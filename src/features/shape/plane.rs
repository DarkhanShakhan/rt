use super::Shape;
use crate::features::{
    consts::EPSILON, material::Material, matrice::Matrice, point::Point, ray::Ray, vector::Vector,
};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Plane {
    id: String,
    transform: Matrice,
    material: Material,
}

impl Plane {
    pub fn new(transform: Matrice, material: Material) -> Self {
        Plane {
            transform,
            material,
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane::new(Matrice::identity_matrix(4), Material::default())
    }
}

impl Shape for Plane {
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

    fn local_normal_at(&self, _world_point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn get_shape_id(&self) -> String {
        self.id.clone()
    }

    fn local_intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        if r.direction.position.y.abs() < EPSILON {
            return None;
        }
        Some(vec![-r.origin.position.y / r.direction.position.y])
    }
}

#[cfg(test)]
mod plane_tests {
    use super::*;
    #[test]
    fn constant_normal_at() {
        let plane = Plane::default();
        let n1 = plane.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = plane.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = plane.local_normal_at(Point::new(-5.0, 0.0, 150.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let plane = Plane::default();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = plane.local_intersect(&r);
        assert_eq!(xs, None);
    }
    #[test]
    fn intersect_coplanar_ray() {
        let plane = Plane::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = plane.local_intersect(&r);
        assert_eq!(xs, None);
    }

    #[test]
    fn ray_intersect_plane_from_above() {
        let plane = Plane::default();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 1.0));
        let xs = plane.local_intersect(&r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn ray_intersect_plane_from_below() {
        let plane = Plane::default();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 1.0));
        let xs = plane.local_intersect(&r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}
