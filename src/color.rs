use std::fmt::Display;

use crate::vec3::Vec3;

const COLOR_BASE: f64 = 255.999;

/// Newtype to make it clear we are working with an (RGB) color.
pub struct Color(Vec3);

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write out the translated rgb color value of each color component [0,255]
        f.write_fmt(format_args!(
            "{} {} {}",
            (self.0.x() * COLOR_BASE) as u8,
            (self.0.y() * COLOR_BASE) as u8,
            (self.0.z() * COLOR_BASE) as u8,
        ))
    }
}
