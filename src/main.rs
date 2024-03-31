use crate::{
    color::Color,
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    discriminant >= 0.
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new_with_data(0., 0., -1.), 0.5, r) {
        return Color::from(Vec3::new_with_data(1., 0., 0.));
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    Color::from(
        (1.0 - a) * Vec3::new_with_data(1., 1., 1.) + a * Vec3::new_with_data(0.5, 0.7, 1.0),
    )
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate image height given the width and aspect ratio.
    // Ensure the height is at least 1
    let image_height = ((image_width as f64 / aspect_ratio).max(1.)) as usize;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new();

    // Caluclate vectors across the horizontal and down the vertical viewport edgess.
    let viewport_u = Vec3::new_with_data(viewport_width, 0., 0.);
    let viewport_v = Vec3::new_with_data(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::new_with_data(0., 0., focal_length)
        - viewport_u / 2.
        - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);

            println!("{pixel_color}");
        }
    }
    eprintln!("\rDone.                   ");
}
