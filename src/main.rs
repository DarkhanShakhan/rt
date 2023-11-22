use std::rc::Rc;

use features::{
    canvas::Canvas,
    color::Color,
    consts::WHITE,
    intersection::{self, hit, Intersection},
    light::{self, Light},
    material::Material,
    point::Point,
    ray::Ray,
    shape::{sphere::Sphere, Shape},
    transormations::{scaling, shearing, translation},
};

mod features;
fn main() {
    draw_sphere()
}

fn draw_sphere() {
    // let ray_origin = Point::new(0.0, 0.0, -5.0);
    // let wall_z = 10.0;
    // let wall_size = 7.0;
    // let canvas_pixels = 400.0;
    // let pixel_size = wall_size / canvas_pixels;
    // let half = wall_size / 2.0;
    // let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    // let mut sphere = Sphere::default();
    // sphere.set_transform(scaling(1.2, 0.3, 0.5) * translation(0.0, 0.5, 0.7));
    // sphere.set_material(Material {
    //     color: Color::new(1.0, 0.2, 1.0),
    //     ..Default::default()
    // });
    // let light = Light::new(Point::new(-10.0, 10.0, -10.0), WHITE);
    // let shape = Rc::new(sphere);
    // for y in 0..canvas_pixels as usize - 1 {
    //     let world_y = half - pixel_size * y as f64;
    //     for x in 0..canvas_pixels as usize - 1 {
    //         let world_x = -half + pixel_size * x as f64;
    //         let position = Point::new(world_x, world_y, wall_z);
    //         let r = Ray::new(ray_origin, (position - ray_origin).normalize());
    //         if let Some(xs) = Intersection::intersects(shape.clone(), &r) {
    //             if let Some(h) = hit(xs) {
    //                 let point = r.position(h.t);
    //                 let normal = h.shape.normal_at(point).unwrap();
    //                 let eye = -r.direction;
    //                 let color = h
    //                     .shape
    //                     .get_material()
    //                     .lighting(&light, &point, &eye, &normal);
    //                 canvas.write_pixel(x, y, color);
    //             }
    //         }
    //     }
    // }
    // canvas.to_ppm()
}
