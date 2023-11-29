// Constants

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) /180.0
}
