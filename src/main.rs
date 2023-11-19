mod color;
mod ray;
mod vec3;

use std::io;

use color::Color;
use ray::Ray;
use vec3::{Vec3, Point3};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
 
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
 
    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {

        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            let pixel_color = Color::new(r, g, b);
            color::write_color(&mut io::stdout(), pixel_color); 
        }
    }

    eprint!("\nDone.\n");
}
