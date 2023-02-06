use std::ops::{Add, BitXor, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Line {
    pub base: Vector,
    pub dir: Vector,
}

impl Vector {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Vector {
        let len = self.len();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: &f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vector> for &f64 {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

// VECTOR PRODUCT

impl BitXor for &Vector {
    type Output = Vector;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.y * rhs.z - self.z - rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// SCALAR PRODUCT

impl Mul for &Vector {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
