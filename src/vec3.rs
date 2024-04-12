#![allow(dead_code)]

use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);

/// Type alias to make code clearer.
pub type Point3 = Vec3;

impl Vec3 {
    /// Instantiate a new `Vec3`.
    pub fn new() -> Self {
        Self(0., 0., 0.)
    }

    /// Generate a new Vec3 with random values in the range [0,1[.
    pub fn random() -> Self {
        Self(rand::random(), rand::random(), rand::random())
    }

    /// Generate a new Vec3 where each field is random within the given bounds
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let dis = Uniform::from(min..max);
        Self(
            dis.sample(&mut rng),
            dis.sample(&mut rng),
            dis.sample(&mut rng),
        )
    }

    /// Generate a random Vec3 which is inside the unit sphere.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            if p.length_squared() <= 1. {
                return p;
            }
        }
    }

    /// Generate a random unit vector.
    pub fn random_unit_vector() -> Self {
        Self::unit_vector(&Self::random_in_unit_sphere())
    }

    /// Create a new `Vec3` with  the given values.
    pub fn new_with_data(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    /// Return the x component of this `Vec3`.
    pub fn x(&self) -> f64 {
        self.0
    }

    /// Return the y component of this `Vec3`.
    pub fn y(&self) -> f64 {
        self.1
    }

    /// Return the z component of this `Vec3`.
    pub fn z(&self) -> f64 {
        self.2
    }

    /// Return the length of this `Vec3` if it is interpreted as a vector.
    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the length of this `Vec3` squared if it is interpreted as a vector.
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Scales the vector to a unit vector.
    pub fn unit_vector(&self) -> Vec3 {
        self / self.len()
    }

    /// Calculate the dot product of 2 vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Calculate the cross vector of 2 vectors.
    pub fn cross(&self, other: &Self) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl Add<Self> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&Self> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<&Self> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<Self> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&Self> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<Self> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<&Self> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut t = *self;
        t *= rhs;
        t
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut t = self;
        t *= rhs;
        t
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let mut t = *rhs;
        t *= self;
        t
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut t = rhs;
        t *= self;
        t
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

impl AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.0, self.1, self.2))
    }
}
