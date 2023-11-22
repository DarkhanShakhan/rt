use std::rc::Rc;

use super::{
    color::Color,
    consts::WHITE,
    intersection::{sort_intersections, Intersection},
    light::Light,
    ray::Ray,
    shape::Shape,
};

pub struct World<S: Shape> {
    pub light: Light,
    pub shapes: Option<Vec<Rc<S>>>,
}

impl<S: Shape> World<S> {
    pub fn new(light: Light, shapes: Option<Vec<Rc<S>>>) -> Self {
        World { light, shapes }
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection<S>>> {
        let mut result = vec![];
        if let Some(shapes) = &self.shapes {
            for s in shapes {
                let mut xs = Intersection::intersects(s.clone(), ray)?;
                result.append(&mut xs)
            }
        }
        if !result.is_empty() {
            sort_intersections(&mut result);
            return Some(result);
        }
        None
    }
}

#[cfg(test)]
mod world_tests {
    use crate::features::{
        material::Material, point::Point, shape::sphere::Sphere, transormations::scaling,
        vector::Vector,
    };

    use super::*;
    #[test]
    fn intersect_world_with_ray() {
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::default();
        s1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        let w = World::new(light, Some(vec![Rc::new(s1), Rc::new(s2)]));
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r).unwrap();
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
