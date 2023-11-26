use super::{
    color::Color,
    computation::Computation,
    consts::BLACK,
    intersection::{hit, sort_intersections, Intersection},
    light::Light,
    material::Material,
    point::Point,
    ray::Ray,
    shape::{sphere::Sphere, Shape},
    transormations::scaling,
};
use std::collections::HashMap;

pub struct World {
    pub light: Light,
    pub objects: HashMap<String, Box<dyn Shape>>,
    pub keys: Vec<String>,
}

impl World {
    pub fn new(light: Light) -> Self {
        World {
            light,
            objects: HashMap::new(),
            keys: vec![],
        }
    }
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        let id = shape.get_shape_id();
        self.objects.insert(id.clone(), shape);
        self.keys.push(id);
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut result = vec![];
        if !self.objects.is_empty() {
            for shape in self.objects.values() {
                if let Some(ixs) = Intersection::intersects(shape.as_ref(), ray) {
                    let mut ixs = ixs;
                    result.append(&mut ixs);
                }
            }
        }
        if !result.is_empty() {
            sort_intersections(&mut result);
            return Some(result);
        }
        None
    }
    pub fn shade_hit(&self, comps: &Computation) -> Color {
        let shape = self.objects.get(&comps.object_id).unwrap();
        shape.get_material().lighting(
            &self.light,
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            self.is_shadowed(&comps.over_point),
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        if let Some(ixs) = self.intersect(ray) {
            if let Some(hit) = hit(ixs) {
                let comps =
                    Computation::new(ray, &hit, self.objects.get(&hit.shape_id).unwrap().as_ref());
                return self.shade_hit(&comps);
            }
        }
        BLACK
    }
    pub fn is_shadowed(&self, point: &Point) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        if let Some(ixs) = self.intersect(&r) {
            if let Some(h) = hit(ixs) {
                if h.t < distance {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new(Light::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let mut s1: Box<dyn Shape> = Box::<Sphere>::default();
        s1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let mut s2: Box<dyn Shape> = Box::<Sphere>::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        w.add_shape(s1);
        w.add_shape(s2);
        w
    }
}

#[cfg(test)]
mod world_tests {
    use crate::features::{computation::Computation, transormations::translation, vector::Vector};

    use super::*;
    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r).unwrap();
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shade_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(&w.keys[0], 4.0);
        let comps = Computation::new(&r, &i, w.objects.get(&w.keys[0]).unwrap().as_ref());
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_from_inside() {
        let w = World {
            light: Light::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)),
            ..Default::default()
        };
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(&w.keys[1], 0.5);
        let comps = Computation::new(&r, &i, w.objects.get(&w.keys[1]).unwrap().as_ref());
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, BLACK);
    }
    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn color_intersection_behind_ray() {
        let mut w = World::default();
        let outer = w.objects.get_mut(&w.keys[0].clone()).unwrap();
        let mut m = outer.get_material();
        m.ambient = 1.0;

        outer.set_material(m);
        let inner = w.objects.get_mut(&w.keys[1].clone()).unwrap();
        let mut m = inner.get_material();
        m.ambient = 1.0;
        let first_color = m.color;
        inner.set_material(m);

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, first_color);
    }
    #[test]
    fn no_shadow() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }
    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let mut w = World::new(Light::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        w.add_shape(Box::<Sphere>::default());
        let mut s = Box::<Sphere>::default();
        s.set_transform(translation(0.0, 0.0, 10.0));
        w.add_shape(s);
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(&w.keys[1], 4.0);
        let comps = Computation::new(&ray, &i, w.objects.get(&w.keys[1]).unwrap().as_ref());
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
