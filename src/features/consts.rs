use super::{color::Color, tuple::Tuple};

pub const EPSILON: f64 = 0.00001;

pub const BLACK: Color = Color {
    rgb: Tuple {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};

pub const WHITE: Color = Color {
    rgb: Tuple {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
};
