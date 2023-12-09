use std::io::Write;

use crate::common;
use crate::vec3::Vec3;

// Type alias for Color 
// for example Color::new(1.0, 0.0, 0.0) is red 
// Color::new(0.0, 1.0, 0.0) is green and so on
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    // Write the translated [0, 225] value of each color component
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    // Write the translated [0, 255] value of each color component 
    writeln!(
        out,
        "{} {} {}", 
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}

