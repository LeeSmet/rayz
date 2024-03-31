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
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.;

            const BASE: f64 = 255.999;

            let ir = (BASE * r) as u8;
            let ig = (BASE * g) as u8;
            let ib = (BASE * b) as u8;

            println!("{ir} {ig} {ib}");
        }
    }
    eprintln!("\rDone.                   ");
}
