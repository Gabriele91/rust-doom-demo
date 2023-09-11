#![allow(dead_code)]
// Using
use core::ops;
use lazy_static::lazy_static;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T : Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn zeros() -> Self where T: Default {
        Vec2 { x: T::default() , y: T::default() }
    }

    pub fn yx(&self) -> Vec2<T> {
        Vec2 { x: self.y, y: self.x }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, right: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + right.x, self.y + right.y)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, right: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x * right.x, self.y * right.y)
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, right: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x - right.x, self.y - right.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T : Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
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
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, right: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + right.x, self.y + right.y, self.z + right.z)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, right: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * right.x, self.y * right.y, self.z * right.z)
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, right: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - right.x, self.y - right.y, self.z - right.z)
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

pub fn no_negative<T : std::cmp::PartialOrd + Default>(value:T) -> T {
    if value < T::default() {
        return T::default();
    }
   return value;
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
