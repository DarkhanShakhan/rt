use crate::features::{
    color::Color,
    consts::{BLACK, WHITE},
    matrice::Matrice,
    point::Point,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Ring {
    a: Color,
    b: Color,
    transform: Matrice,
}

impl Ring {
    pub fn new(a: Color, b: Color, transform: Matrice) -> Ring {
        Ring { a, b, transform }
    }
    pub fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }

    pub fn at(&self, point: &Point) -> Color {
        if (point.position.x * point.position.x + point.position.z * point.position.z).sqrt() as i32
            % 2
            == 0
        {
            return self.a;
        }
        self.b
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self::new(WHITE, BLACK, Matrice::identity_matrix(4))
    }
}
