#![allow(dead_code)]
// Using
use core::ops;
use lazy_static::lazy_static;
use std::f32::consts::PI;
use num_traits::{cast::NumCast, Float};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T : Sized + Copy + NumCast> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn new_x(x: T) -> Self where T: Default {
        Vec2 { x: x, y: T::default() }
    }

    pub fn new_y(y: T) -> Self where T: Default {
        Vec2 { x: T::default(), y: y }
    }

    pub fn zeros() -> Self where T: Default {
        Vec2 { x: T::default() , y: T::default() }
    }

    pub fn yx(&self) -> Vec2<T> {
        Vec2 { x: self.y, y: self.x }
    }

    pub fn as_vec<U: Sized + Copy + NumCast + Default>(&self) -> Vec2<U> {
        Vec2::<U>{ 
            x: NumCast::from(self.x).unwrap_or_default(),
            y: NumCast::from(self.y).unwrap_or_default(),
        }
    }

}

impl<T: Float> Vec2<T> {
    pub fn normalize(&self) -> Vec2<T> {
        let length = self.dot(&self).sqrt();
        Vec2 { x: self.x / length, y: self.y / length }
    }
    
    pub fn distance(&self, right: &Vec2<T>) -> T {
        let diff = *self - *right;
        diff.dot(&diff).sqrt()
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Sized + Copy + NumCast> Vec2<T> {
    pub fn dot(&self, right: &Vec2<T>) -> T {
        return self.x * right.x + self.y * right.y;
    }

    pub fn cross(&self, right: &Vec2<T>) -> T {
        self.x * right.y - self.y * right.x
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, right: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + right.x, self.y + right.y)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, right: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x * right.x, self.y * right.y)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, right: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x - right.x, self.y - right.y)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, right: T) -> Vec2<T> {
        Vec2::new(self.x + right, self.y + right)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, right: T) -> Vec2<T> {
        Vec2::new(self.x * right, self.y * right)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, right: T) -> Vec2<T> {
        Vec2::new(self.x - right, self.y - right)
    }
}


impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, right: Vec2<T>) {
        *self = Vec2::new(self.x + right.x, self.y + right.y);
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, right: Vec2<T>) {
        *self = Vec2::new(self.x * right.x, self.y * right.y);
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, right: Vec2<T>) {
        *self = Vec2::new(self.x - right.x, self.y - right.y);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T : Sized + Copy + NumCast> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_vec2_z(xy: &Vec2<T>, z: T) -> Self {
        Vec3 { x: xy.x, y: xy.y, z: z }
    }

    pub fn new_x(x: T) -> Self where T: Default {
        Vec3 { x: x, y: T::default(), z: T::default() }
    }

    pub fn new_y(y: T) -> Self where T: Default {
        Vec3 { x: T::default(), y: y, z: T::default() }
    }

    pub fn new_z(z: T) -> Self where T: Default {
        Vec3 { x: T::default(), y: T::default(), z: z }
    }
    
    pub fn zeros() -> Self where T: Default {
        Vec3 { x: T::default(), y: T::default(), z: T::default() }
    }

    pub fn xy(&self) -> Vec2<T> {
        Vec2::new( self.x, self.y )
    }

    pub fn xz(&self) -> Vec2<T> {
        Vec2::new( self.x, self.z )
    }

    pub fn yx(&self) -> Vec2<T> {
        Vec2::new( self.y, self.x )
    }

    pub fn yz(&self) -> Vec2<T> {
        Vec2::new( self.y, self.z )
    }

    pub fn zx(&self) -> Vec2<T> {
        Vec2::new( self.z, self.x )
    }

    pub fn zy(&self) -> Vec2<T> {
        Vec2::new( self.z, self.y )
    }

    pub fn as_vec< U:  Sized + Copy + NumCast + Default>(&self) -> Vec3<U> {
        Vec3::<U>{ 
            x: NumCast::from(self.x).unwrap_or_default(),
            y: NumCast::from(self.y).unwrap_or_default(),
            z: NumCast::from(self.z).unwrap_or_default(),
        }
    }

}

impl<T: Float> Vec3<T> {
    pub fn normalize(&self) -> Vec3<T> {
        let length = self.dot(&self).sqrt();
        Vec3 { x: self.x / length, y: self.y / length, z: self.z / length }
    }

    pub fn distance(&self, right: &Vec3<T>) -> T {
        let diff = *self - *right;
        diff.dot(&diff).sqrt()
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Sized + Copy + NumCast> Vec3<T> {
    pub fn dot(&self, right: &Vec3<T>) -> T {
        return self.x * right.x + self.y * right.y + self.z * right.z;
    }

    pub fn cross(&self, right: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * right.z - self.z * right.y, 
            self.z * right.x - self.x * right.z, 
            self.x * right.y - self.y * right.x
        )
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, right: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + right.x, self.y + right.y, self.z + right.z)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, right: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * right.x, self.y * right.y, self.z * right.z)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, right: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - right.x, self.y - right.y, self.z - right.z)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::Add<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, right: T) -> Vec3<T> {
        Vec3::new(self.x + right, self.y + right, self.z + right)
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, right: T) -> Vec3<T> {
        Vec3::new(self.x * right, self.y * right, self.z * right)
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::Sub<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, right: T) -> Vec3<T> {
        Vec3::new(self.x - right, self.y - right, self.z - right)
    }
}

impl<T: ops::Add<Output = T> + Sized + Copy + NumCast> ops::AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, right: Vec3<T>) {
        *self = Vec3::new(self.x + right.x, self.y + right.y, self.z + right.z);
    }
}

impl<T: ops::Mul<Output = T> + Sized + Copy + NumCast> ops::MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, right: Vec3<T>) {
        *self = Vec3::new(self.x * right.x, self.y * right.y, self.z * right.z);
    }
}

impl<T: ops::Sub<Output = T> + Sized + Copy + NumCast> ops::SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, right: Vec3<T>) {
        *self = Vec3::new(self.x - right.x, self.y - right.y, self.z - right.z);
    }
}

pub fn max<T : std::cmp::PartialOrd>(value1:T, value2: T) -> T {
    if value1 < value2 {
        return value2;
    }
   return value1;
}

pub fn min<T : std::cmp::PartialOrd>(value1:T, value2: T) -> T {
    if value2 < value1 {
        return value2;
    }
   return value1;
}

pub fn clamp<T : std::cmp::PartialOrd>(value:T,min:T,max:T) -> T {
    if value < min {
        return min;
    } else if value > max {
        return max;
    }
   return value;
}

pub fn lerp<T: Float>(start: T, end: T, alpha: T) -> T
{
    start + (end - start) * alpha
}

pub fn no_negative<T : std::cmp::PartialOrd + Default>(value:T) -> T {
    if value < T::default() {
        return T::default();
    }
   return value;
}

pub fn radians<T: Float + NumCast + Default>(degrees: T) -> T {
    let pi: T = T::from(std::f64::consts::PI).unwrap_or_default();
    degrees * (pi / T::from(180.0).unwrap())
}

pub fn degrees<T: Float + NumCast + Default>(radians: T) -> T {
    let pi: T = T::from(std::f64::consts::PI).unwrap_or_default();
    radians * (T::from(180.0).unwrap() / pi)
}

lazy_static! {
    
    pub static ref SIN: [f32; 360] = {
        let mut sin_values = [0.0; 360];
        for i in 0..360 {
            sin_values[i] = (i as f32 * PI / 180.0).sin();
        }
        sin_values
    };
    
    pub static ref COS: [f32; 360] = {
        let mut cos_values = [0.0; 360];
        for i in 0..360 {
            cos_values[i] = (i as f32 * PI / 180.0).cos();
        }
        cos_values
    };

}
