
use bytemuck;

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d {

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// lerp, but chopped at t = 0 and t = 1
    #[inline(always)]
    pub fn lerp_with_chop(&self, other: Self, t: f32) -> Self {
        if t > 1.0 {return *self}
        if t < 0.0 {return other}
        *self*t + other*(1.0-t)
    }

    /// returns self*t + other*(1-t)
    #[inline(always)]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        *self*t + other*(1.0-t)
    }

    #[inline(always)]
    pub fn size(&self) -> f32 {
        (self.dot(*self)).sqrt()
    }

    #[inline(always)]
    pub fn unit_assign(&mut self) {
        *self *= 1.0/self.size();
    }

    #[inline(always)]
    pub fn unit(&self) -> Self {
        *self * (1.0/self.size())
    }

    #[inline(always)]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn len_sq(&self) -> f32 {
        self.dot(*self)
    }
    
    #[inline(always)]
    pub fn cross(&self, other: Self) -> Self {
        Vec3d {
            x: self.y*other.z - other.y*self.z,
            y: - self.x*other.z + other.x*self.z,
            z: self.x*other.y - other.x*self.y,
        }
}

    /// element wise multiplicatin of 2 vectors
    #[inline(always)]
    pub fn mul(&self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Add for Vec3d {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3d {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// // cross makes more sense here cuz vec1*vec2 -> Vec3d for 3d vectors
// impl Mul for Vec3d {
//     type Output = Self;

//     /// 3d vector multiplication
//     #[inline(always)]
//     fn mul(self, other: Self) -> Self {
//         Vec3d {
//             x: self.y*other.z - other.y*self.z,
//             y: - self.x*other.z + other.x*self.z,
//             z: self.x*other.y - other.x*self.y,
//         }
//     }
// }

impl Mul<f32> for Vec3d {
    type Output = Self;

    #[inline(always)]
    fn mul(self, t: f32) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}
impl AddAssign for Vec3d {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}
impl SubAssign for Vec3d {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}
impl MulAssign<f32> for Vec3d {
    #[inline(always)]
    fn mul_assign(&mut self, t: f32) {
        self.x = self.x * t;
        self.y = self.y * t;
        self.z = self.z * t;
    }
}
