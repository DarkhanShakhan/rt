use super::{matrice::Matrice, ray::Ray};

pub mod sphere;

pub trait Shape {
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>>;
    fn set_transform(&mut self, transform: Matrice);
}
