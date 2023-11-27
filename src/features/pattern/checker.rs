use crate::features::{
    color::Color,
    consts::{BLACK, WHITE},
    matrice::Matrice,
    point::Point,
};

use super::Pattern;

pub struct Checker {
    a: Color,
    b: Color,
    transform: Matrice,
}

impl Checker {
    pub fn new(a: Color, b: Color, transform: Matrice) -> Self {
        Self { a, b, transform }
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self::new(WHITE, BLACK, Matrice::identity_matrix(4))
    }
}

impl Pattern for Checker {
    fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }

    fn at(&self, point: &Point) -> Color {
        if (point.position.x.abs() + point.position.y.abs() + point.position.z.abs()) as i32 % 2
            == 0
        {
            return self.a;
        }
        self.b
    }
}
