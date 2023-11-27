use crate::features::{
    color::Color,
    consts::{BLACK, WHITE},
    matrice::Matrice,
    point::Point,
};

use super::Pattern;

pub struct Stripe {
    a: Color,
    b: Color,
    transform: Matrice,
}

impl Stripe {
    pub fn new(a: Color, b: Color, transform: Matrice) -> Stripe {
        Self { a, b, transform }
    }
}

impl Default for Stripe {
    fn default() -> Self {
        Self::new(WHITE, BLACK, Matrice::identity_matrix(4))
    }
}

impl Pattern for Stripe {
    fn get_transform(&self) -> Matrice {
        self.transform.clone()
    }

    fn at(&self, point: &Point) -> Color {
        if point.position.x.floor() as i32 % 2 == 0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod stripe_tests {
    use super::*;
    #[test]
    fn new() {
        let p = Stripe::new(WHITE, BLACK, Matrice::identity_matrix(4));
        assert_eq!(p.a, WHITE);
        assert_eq!(p.b, BLACK);
    }

    #[test]
    fn constant_in_y() {
        let p = Stripe::default();
        assert_eq!(p.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.at(&Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(p.at(&Point::new(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn constant_in_z() {
        let p = Stripe::default();
        assert_eq!(p.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.at(&Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(p.at(&Point::new(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn alternate_in_x() {
        let p = Stripe::default();
        assert_eq!(p.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(p.at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    }
}
