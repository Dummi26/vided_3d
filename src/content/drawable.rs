use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

use crate::pos::{Line, Vector};

pub trait Drawable: Sync + Send {
    /// given a ray of light, returns nothing if the line misses the drawable object, or the distance (in lengths of the line's direction vector) from the line's base vector to the intersection.
    fn get_intersection(&self, ray: &Line) -> Option<(f64, PointRayProperties)>;
}

#[derive(Clone)]
pub struct PointRayProperties {
    pub orientation: Vector,
    pub scattering_orientations: Vec<Vector>,
    pub light_properties: PointLightProperties,
}

#[derive(Clone)]
pub struct PointLightPropertiesCustomType<T> {
    pub emittance: T,
    pub transparency: T,
    pub reflectiveness: T,
    pub scattering: T,
}
impl<T> PointLightPropertiesCustomType<T>
where
    T: Clone,
{
    pub fn convert<F, U>(self, f: F) -> PointLightPropertiesCustomType<U>
    where
        F: Fn(T) -> U,
    {
        PointLightPropertiesCustomType {
            emittance: f(self.emittance),
            transparency: f(self.transparency),
            reflectiveness: f(self.reflectiveness),
            scattering: f(self.scattering),
        }
    }
    pub fn convert_ref<F, U>(&self, f: F) -> PointLightPropertiesCustomType<U>
    where
        F: Fn(&T) -> U,
    {
        PointLightPropertiesCustomType {
            emittance: f(&self.emittance),
            transparency: f(&self.transparency),
            reflectiveness: f(&self.reflectiveness),
            scattering: f(&self.scattering),
        }
    }
}

pub type PointLightProperties = PointLightPropertiesCustomType<Color>;

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
    pub fn all(v: f64) -> Self {
        Self {
            r: v,
            g: v,
            b: v,
            a: v,
        }
    }
    pub fn rgba(v: f64, a: f64) -> Self {
        Self {
            r: v,
            g: v,
            b: v,
            a,
        }
    }
    pub fn rgb(v: f64) -> Self {
        Self {
            r: v,
            g: v,
            b: v,
            a: 1.0,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.a == 0.0 && self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }

    pub fn from_u8s(u8s: [u8; 4]) -> Self {
        Self {
            r: u8s[0] as f64 / 255.0,
            g: u8s[1] as f64 / 255.0,
            b: u8s[2] as f64 / 255.0,
            a: u8s[3] as f64 / 255.0,
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
impl MulAssign<&Self> for Color {
    fn mul_assign(&mut self, rhs: &Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
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
impl Div for &Color {
    type Output = Color;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}
impl Div<f64> for &Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}
