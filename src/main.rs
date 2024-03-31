use crate::{
    color::Color,
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0. {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new_with_data(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new_with_data(0., 0., -1.)).unit_vector();
        return Color::from(0.5 * Vec3::new_with_data(n.x() + 1., n.y() + 1., n.z() + 1.));
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
