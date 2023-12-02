use crate::features::{
    color::Color,
    consts::{BLACK, WHITE},
    matrice::Matrice,
    point::Point,
};

#[derive(Debug, PartialEq, Clone)]
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
    pub fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }

    pub fn at(&self, point: &Point) -> Color {
        let distance = self.to - self.from;
        let fraction = point.position.x - point.position.x.floor();
        self.from + distance * fraction
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::new(WHITE, BLACK, Matrice::identity_matrix(4))
    }
}
