mod color;
mod ray;
mod vec3;

use std::io;

use color::Color;
use ray::Ray;
use vec3::{Vec3, Point3};

// To tell if the ray hits the sphere or not (retuns a boolean value)
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = vec3::dot(r.direction(), r.direction());
    let b = 2.0 * vec3::dot(oc, r.direction());
    let c = vec3::dot(oc, oc) - (radius * radius);
    let discriminant  = (b * b) - (4.0 * a * c);
    discriminant >= 0.0
}

// To determine the color of the ray
fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    } 

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = 
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {

        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprint!("\nDone.\n");
}
