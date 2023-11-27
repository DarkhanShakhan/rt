use super::{material::Material, matrice::Matrice, point::Point, ray::Ray, vector::Vector};

pub mod plane;
pub mod sphere;
pub trait Shape {
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        self.local_intersect(&r.transform(&self.get_transform().inverse()?))
    }
    fn local_intersect(&self, r: &Ray) -> Option<Vec<f64>>;
    fn set_transform(&mut self, transform: Matrice);
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> Material;
    fn get_transform(&self) -> Matrice;
    fn normal_at(&self, world_point: Point) -> Option<Vector> {
        let local_point = self.get_transform().inverse()? * world_point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = self.get_transform().inverse()?.transpose() * local_normal;
        Some(world_normal.normalize())
    }
    fn local_normal_at(&self, local_point: Point) -> Vector;
    fn get_shape_id(&self) -> String;
}
