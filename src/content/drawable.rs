use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::pos::{Line, Vector};

pub trait Drawable {
    /// returns a sphere (point and radius) that encompasses the entirity of the drawable.
    fn get_outer_bounds(&self) -> (Vector, f64);
    /// given a ray of light, returns nothing if the line misses the drawable object, or the distance (in lengths of the line's direction vector) from the line's base vector to the intersection.
    fn get_intersection(&self, ray: &Line) -> Option<(f64, PointRayProperties)>;
}

#[derive(Clone)]
pub struct PointRayProperties {
    pub orientation: Vector,
    pub light_properties: PointLightProperties,
}

#[derive(Clone)]
pub struct PointLightProperties {
    pub emittance: Color,
    pub transparency: Color,
    pub reflectiveness: Color,
    pub scattering: Color,
}

#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
impl Color {
    pub fn transparent() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    pub fn u8s(&self) -> [u8; 4] {
        [
            (self.r.max(0.0).min(1.0) * 255.0) as u8,
            (self.g.max(0.0).min(1.0) * 255.0) as u8,
            (self.b.max(0.0).min(1.0) * 255.0) as u8,
            255, // (self.a.max(0.0).min(1.0) * 255.0) as u8,
        ]
    }
}
impl Add for &Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}
impl AddAssign<&Self> for Color {
    fn add_assign(&mut self, rhs: &Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}
impl Mul for &Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}
impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}
