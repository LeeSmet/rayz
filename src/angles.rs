#![allow(dead_code)]

/// Convert degrees to the radian representation.
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
