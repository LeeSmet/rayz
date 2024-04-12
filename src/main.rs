use std::rc::Rc;

use camera::Camera;
use hittable_list::HittableList;

use crate::{sphere::Sphere, vec3::Point3};

mod angles;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let camera = Camera::new(aspect_ratio, image_width, camera::ANTI_ALIASING_STRONG, 50);

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point3::new_with_data(0., 0., -1.),
        0.5,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new_with_data(0., -100.5, -1.),
        100.,
    )));

    camera.render(&world);
}
