use crate::{color::Color, vec3::Vec3};

mod color;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::from(Vec3::new_with_data(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.,
            ));

            println!("{pixel_color}");
        }
    }
    eprintln!("\rDone.                   ");
}
