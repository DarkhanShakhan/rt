use self::sphere::Sphere;

use super::{material::Material, matrice::Matrice, point::Point, ray::Ray, vector::Vector};

pub mod sphere;

pub trait Shape {
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>>;
    fn set_transform(&mut self, transform: Matrice);
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> Material;
    fn get_transform(&self) -> Matrice;
    fn normal_at(&self, world_point: Point) -> Option<Vector>;
}

pub fn default_sphere() -> Box<dyn Shape> {
    Box::new(Sphere::default())
}
