use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{point::Point, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector {
    pub position: Tuple,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            position: Tuple::new(x, y, z),
        }
    }
    pub fn magnitude(&self) -> f64 {
        self.position.magnitude()
    }

    pub fn normalize(&self) -> Self {
        Self::from(self.position / self.magnitude())
    }

    pub fn dot_product(&self, rhs: &Vector) -> f64 {
        self.position.dot(&rhs.position)
    }

    pub fn cross_product(&self, rhs: &Vector) -> Self {
        Self::from(Tuple::new(
            self.position.y * rhs.position.z - self.position.z * rhs.position.y,
            self.position.z * rhs.position.x - self.position.x * rhs.position.z,
            self.position.x * rhs.position.y - self.position.y * rhs.position.x,
        ))
    }
}

impl From<Tuple> for Vector {
    fn from(value: Tuple) -> Self {
        Vector { position: value }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::from(self.position + rhs.position)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::from(self.position + rhs.position)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::from(self.position - rhs.position)
    }
}

impl Sub<Point> for Vector {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point::from(self.position - rhs.position)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector::from(-self.position)
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::from(self.position / rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::from(self.position * rhs)
    }
}
