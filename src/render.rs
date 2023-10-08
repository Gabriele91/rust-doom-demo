
#![allow(dead_code)]
// Using, d3d
use crate::player::Player;
use crate::consts::{self, H_WIDTH, H_FOV};
use crate::math::radians;
// Using
use lazy_static::lazy_static;
use pixels::Pixels;
use std::f32::consts::PI;
use libm::atanf;

lazy_static! {
    pub static ref SCREEN_DIST: f32 = {
        (consts::H_WIDTH as f32) / radians(H_FOV).tan()
    };

    pub static ref X_TO_ANGLE: [f32; (consts::WIDTH+1) as usize] = {
        let mut x_to_angle = [0.0; (consts::WIDTH+1) as usize];
        for x in 0..=consts::WIDTH {
            x_to_angle[x as usize] = atanf((consts::H_WIDTH - x) as f32 / *SCREEN_DIST);
        }
        x_to_angle
    };
}

#[inline]
pub fn inv_fov() -> f32 {
    1.0 / (consts::H_FOV * PI / 180.0).tan()
}

#[inline]
pub fn width_on_fov() -> i32 {
    ((consts::WIDTH as f32) * inv_fov()) as i32
} 

pub fn angle_to_x(angle: f32) -> f32 {
    if angle > 0.0 {
        *SCREEN_DIST - angle.tan() * (H_WIDTH as f32)
    } else {
        -angle.tan() * (H_WIDTH as f32) + *SCREEN_DIST
    }
}

pub trait Render {
    fn draw(&mut self, pixels: &mut Pixels, player: &Player);
}