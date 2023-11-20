use std::io::Write;

use crate::vec3::Vec3;

// Type alias for Color 
// for example Color::new(1.0, 0.0, 0.0) is red 
// Color::new(0.0, 1.0, 0.0) is green and so on
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // Write the translated [0, 225] value of each color component
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}

