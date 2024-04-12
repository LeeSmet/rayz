use crate::{
    color::SampledColor,
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// Strong 16x anti aliasing.
pub const ANTI_ALIASING_STRONG: usize = 16;

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
    /// Amount of samples to generate per pixel (anti aliasing)
    samples_per_pixel: usize,
    /// Maximum amount of times we bounce a ray
    max_depth: usize,
}

impl Camera {
    /// Create a new `Camera` with the proivded aspect ratio and output image width, the maximum
    /// amount of samples per pixel, and the maximum amount of times a single ray can bounce.
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
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
            samples_per_pixel,
            max_depth,
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
                let mut color = Vec3::new();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += &Self::ray_color(&ray, self.max_depth, &world);
                }
                let pixel_color = SampledColor::new(color, self.samples_per_pixel);

                println!("{pixel_color}");
            }
        }
        eprintln!("\rDone.                      ");
    }

    fn ray_color(r: &Ray, depth: usize, world: &HittableList) -> Vec3 {
        // Ray bounce limit, no more light
        if depth == 0 {
            return Vec3::new();
        }
        // Check if we have a collision first.
        // tmin 0 as we don't want to look behind the viewport, infinity max as we look infinitely far
        // in the distance.
        if let Some(hit_record) = world.hit(r, Interval::new(0., f64::INFINITY)) {
            let direction = Vec3::random_on_hemisphere(&hit_record.normal);
            return 0.5 * Self::ray_color(&Ray::new(hit_record.p, direction), depth - 1, world);
        }

        // Background
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Vec3::new_with_data(1., 1., 1.) + a * Vec3::new_with_data(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Return a random point in the square surrounding the origin pixel.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
