use self::{checker::Checker, gradient::Gradient, ring::Ring, stripe::Stripe};

use super::{
    color::Color,
    matrice::Matrice,
    point::{self, Point},
    shape::Shape,
};
pub mod checker;
pub mod gradient;
pub mod ring;
pub mod stripe;
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Stripe(Stripe),
    Checker(Checker),
    Ring(Ring),
    Gradient(Gradient),
}

impl Pattern {
    pub fn at_obj(&self, shape: &dyn Shape, point: &Point) -> Option<Color> {
        let object_point = shape.get_transform().inverse()? * *point;
        let pattern_point = self.get_transform().inverse()? * object_point;
        let res_color = match self {
            Pattern::Stripe(s) => s.at(&pattern_point),
            Pattern::Checker(c) => c.at(&pattern_point),
            Pattern::Ring(r) => r.at(&pattern_point),
            Pattern::Gradient(g) => g.at(&pattern_point),
        };
        Some(res_color)
    }
    pub fn get_transform(&self) -> Matrice {
        match self {
            Pattern::Stripe(s) => s.get_transform(),
            Pattern::Checker(c) => c.get_transform(),
            Pattern::Ring(r) => r.get_transform(),
            Pattern::Gradient(g) => g.get_transform(),
        }
    }
    pub fn at(&self, point: &Point) -> Color {
        match self {
            Pattern::Stripe(s) => s.at(point),
            Pattern::Checker(c) => c.at(point),
            Pattern::Ring(r) => r.at(point),
            Pattern::Gradient(g) => g.at(point),
        }
    }
}
