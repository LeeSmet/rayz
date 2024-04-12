use std::fmt::Display;

use crate::{interval::Interval, vec3::Vec3};

const COLOR_BASE: f64 = 255.999;

/// Newtype to make it clear we are working with an (RGB) color.
pub struct Color(Vec3);

/// Color composed of multiple samples
pub struct SampledColor {
    color: Vec3,
    samples: usize,
}

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

impl SampledColor {
    /// Create a new SampledColor
    pub fn new(color: Vec3, samples: usize) -> Self {
        Self { color, samples }
    }
}

impl Display for SampledColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = 1.0 / self.samples as f64;

        let r = linear_to_gamma(self.color.x() * scale);
        let g = linear_to_gamma(self.color.y() * scale);
        let b = linear_to_gamma(self.color.z() * scale);

        let interval_intensity = Interval::new(0.000, 0.999);
        const COLOR_BASE: f64 = 256.;
        f.write_fmt(format_args!(
            "{} {} {}",
            (interval_intensity.clamp(r) * COLOR_BASE) as u8,
            (interval_intensity.clamp(g) * COLOR_BASE) as u8,
            (interval_intensity.clamp(b) * COLOR_BASE) as u8,
        ))
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
