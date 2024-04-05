#![allow(dead_code)]

/// A closed interval
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    const WORLD: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);

    /// Create a new `Interval`.
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Checks if a value is in the `Interval`.
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    /// Checks if a value is surrounded by the `Interval`, that is, it is part of the interval and
    /// not equal ot the min or the max value.
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    /// Clamp the value in the interval
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}
