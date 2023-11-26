use std::f64::consts::PI;

use crate::features::{
    camera::Camera,
    color::Color,
    consts::WHITE,
    light::Light,
    material::Material,
    point::Point,
    shape::sphere::Sphere,
    transormations::{rotation_x, rotation_y, scaling, translation, view_transformation},
    vector::Vector,
    world::World,
};

pub fn draw_sphere() {
    let material = Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };
    let floor = Sphere::new(scaling(10.0, 0.01, 10.0), material.clone());
    let left_wall = Sphere::new(
        translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
        material.clone(),
    );
    let right_wall = Sphere::new(
        translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0),
        material.clone(),
    );
    let middle = Sphere::new(
        translation(-0.5, 1.0, 0.5),
        Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: (0.3),
            ..Default::default()
        },
    );
    let right = Sphere::new(
        translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );
    let left = Sphere::new(
        translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        Material {
            color: Color::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );
    //add shapes function
    let mut world = World::new(Light::new(Point::new(-10.0, 10.0, -10.0), WHITE));
    world.add_shape(Box::new(floor));
    world.add_shape(Box::new(left_wall));
    world.add_shape(Box::new(right_wall));
    world.add_shape(Box::new(middle));
    world.add_shape(Box::new(right));
    world.add_shape(Box::new(left));
    let mut camera = Camera::new(800.0, 400.0, PI / 3.0);
    camera.transform = view_transformation(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    camera.render(&world).to_ppm();
}
