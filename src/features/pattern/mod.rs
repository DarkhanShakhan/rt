use super::{color::Color, matrice::Matrice, point::Point, shape::Shape};
pub mod checker;
pub mod gradient;
pub mod ring;
pub mod stripe;
pub trait Pattern {
    fn at_obj(&self, shape: &dyn Shape, point: &Point) -> Option<Color> {
        let object_point = shape.get_transform().inverse()? * *point;
        let pattern_point = self.get_transform().inverse()? * object_point;
        Some(self.at(&pattern_point))
    }
    fn get_transform(&self) -> Matrice;
    fn at(&self, point: &Point) -> Color;
}
