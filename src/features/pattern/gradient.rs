use crate::features::{
    color::Color,
    consts::{BLACK, WHITE},
    matrice::Matrice,
    point::Point,
};

use super::Pattern;

pub struct Gradient {
    from: Color,
    to: Color,
    transform: Matrice,
}

impl Gradient {
    pub fn new(from: Color, to: Color, transform: Matrice) -> Self {
        Self {
            from,
            to,
            transform,
        }
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::new(WHITE, BLACK, Matrice::identity_matrix(4))
    }
}

impl Pattern for Gradient {
    fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }

    fn at(&self, point: &Point) -> Color {
        let distance = self.to - self.from;
        let fraction = point.position.x - point.position.x.floor();
        self.from + distance * fraction
    }
}
