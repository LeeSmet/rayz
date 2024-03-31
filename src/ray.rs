#![allow(dead_code)]

use crate::vec3::{Point3, Vec3};

/// A single ray
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Create a new `Ray` strating in the given origin and moving in the given direction.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get the origin of this `Ray`.
    pub const fn origin(&self) -> Point3 {
        self.origin
    }

    /// Get the direction of this `Ray`.
    pub const fn direction(&self) -> Point3 {
        self.direction
    }

    /// Calculate the position of a `Ray` after a given time.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
