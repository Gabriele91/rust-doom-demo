// Using, d3d
use crate::math::{clamp, Vec2, Vec3};
// Using
use std::vec;

pub struct Wall {
    pub point1: Vec2<i32>,
    pub point2: Vec2<i32>,
    pub color: [u8; 4],
}

pub struct Sector {
    pub wall: Vec2<i32>,
    pub height: Vec2<i32>,
    pub center: Vec2<i32>,
    pub distance: i32,
}

impl Sector {
    pub fn new(wall: &Vec2<i32>, height: &Vec2<i32>) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            center: Vec2::new(0, 0),
            distance: 0,
        }
    }
}

pub struct Wolrd {
    pub walls: Vec<Wall>,
    pub sectors: Vec<Sector>,
}
