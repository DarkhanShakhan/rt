use super::{
    color::Color,
    consts::{BLACK, WHITE},
    light::Light,
    point::Point,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
    pub fn lighting(&self, light: &Light, point: &Point, eyev: &Vector, normalv: &Vector) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - *point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot_product(normalv);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflectv_dot_eye = reflectv.dot_product(eyev);
            if reflectv_dot_eye <= 0.0 {
                specular = BLACK;
            } else {
                let factor = reflectv_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(WHITE, 0.1, 0.9, 0.9, 200.0)
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}

#[cfg(test)]
mod lighting_tests {
    use super::*;

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn eye_between_light_and_surface_offset_45() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0));
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0))
    }
    #[test]
    fn eye_opposite_surface_offset_45() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364))
    }
    #[test]
    fn eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, -(2.0_f64.sqrt() / 2.0), -(2.0_f64.sqrt() / 2.0));
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364))
    }
    #[test]
    fn light_behind_surface() {
        let m = Material::default();
        let position = Point::default();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1))
    }
}
