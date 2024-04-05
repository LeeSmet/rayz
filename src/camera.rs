use crate::{
    color::Color,
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    _aspect_ratio: f64,
    /// Rendered image width
    image_width: usize,
    /// Rendered image height
    image_height: usize,
    /// Camera center
    center: Point3,
    /// Location of pixel 0,0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
}

impl Camera {
    /// Create a new `Camera` with the proivded aspect ratio and output image width.
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        // Calculate image height given the width and aspect ratio.
        // Ensure the height is at least 1
        let image_height = ((image_width as f64 / aspect_ratio).max(1.)) as usize;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new();

        // Caluclate vectors across the horizontal and down the vertical viewport edgess.
        let viewport_u = Vec3::new_with_data(viewport_width, 0., 0.);
        let viewport_v = Vec3::new_with_data(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new_with_data(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            _aspect_ratio: aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    /// Render a world as seen through this `Camera`.
    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}   ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, &world);

                println!("{pixel_color}");
            }
        }
        eprintln!("\rDone.                      ");
    }

    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        // Check if we have a collision first.
        // tmin 0 as we don't want to look behind the viewport, infinity max as we look infinitely far
        // in the distance.
        if let Some(hit_record) = world.hit(r, Interval::new(0., f64::INFINITY)) {
            return Color::from(0.5 * (hit_record.normal + Vec3::new_with_data(1., 1., 1.)));
        }

        // Background
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Color::from(
            (1.0 - a) * Vec3::new_with_data(1., 1., 1.) + a * Vec3::new_with_data(0.5, 0.7, 1.0),
        )
    }
}
