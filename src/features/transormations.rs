use super::{matrice::Matrice, tuple::Tuple, vector::Vector};

pub fn translation(x: f64, y: f64, z: f64) -> Matrice {
    let mut out = Matrice::identity_matrix(4);
    out.write_element(0, 3, x);
    out.write_element(1, 3, y);
    out.write_element(2, 3, z);
    out
}
pub fn scaling(x: f64, y: f64, z: f64) -> Matrice {
    let mut out = Matrice::identity_matrix(4);
    out.write_element(0, 0, x);
    out.write_element(1, 1, y);
    out.write_element(2, 2, z);
    out
}

pub fn rotation_x(rad: f64) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(1, 1, rad.cos());
    matrice.write_element(1, 2, -rad.sin());
    matrice.write_element(2, 1, rad.sin());
    matrice.write_element(2, 2, rad.cos());
    matrice
}

pub fn rotation_y(rad: f64) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 0, rad.cos());
    matrice.write_element(0, 2, rad.sin());
    matrice.write_element(2, 0, -rad.sin());
    matrice.write_element(2, 2, rad.cos());
    matrice
}

pub fn rotation_z(rad: f64) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 0, rad.cos());
    matrice.write_element(0, 1, -rad.sin());
    matrice.write_element(1, 0, rad.sin());
    matrice.write_element(1, 1, rad.cos());
    matrice
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 1, xy);
    matrice.write_element(0, 2, xz);
    matrice.write_element(1, 0, yx);
    matrice.write_element(1, 2, yz);
    matrice.write_element(2, 0, zx);
    matrice.write_element(2, 1, zy);
    matrice
}

pub fn view_transformation(from: Vector, to: Vector, up: Vector) -> Matrice {
    let forward = (to - from).normalize();
    let left = forward.cross_product(&up.normalize());
    let true_up = left.cross_product(&forward);
    Matrice {
        size: 4,
        matrice: vec![
            vec![left.position.x, left.position.y, left.position.z, 0.0],
            vec![
                true_up.position.x,
                true_up.position.y,
                true_up.position.z,
                0.0,
            ],
            vec![
                -forward.position.x,
                -forward.position.y,
                -forward.position.z,
                0.0,
            ],
            vec![0.0, 0.0, 0.0, 1.0],
        ],
    } * translation(-from.position.x, -from.position.y, -from.position.z)
}

#[cfg(test)]
mod view_transformation_tests {

    use super::*;

    #[test]
    fn test_default_view() {
        let from = Vector::new(0.0, 0.0, 0.0);
        let to = Vector::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        assert_eq!(t, Matrice::identity_matrix(4))
    }

    #[test]
    fn test_positive_z_direction() {
        let from = Vector::new(0.0, 0.0, 0.0);
        let to = Vector::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0))
    }

    #[test]
    fn test_moving_world() {
        let from = Vector::new(0.0, 0.0, 8.0);
        let to = Vector::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        assert_eq!(t, translation(0.0, 0.0, -8.0))
    }

    #[test]
    fn test_arbitrary_view() {
        let from = Vector::new(1.0, 3.0, 2.0);
        let to = Vector::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = view_transformation(from, to, up);
        println!("{:?}", t);
    }
}
