#![allow(dead_code)]
// Using, d3d
use crate::consts;
use crate::math::Vec2;

pub struct Wall {
    pub point1: Vec2<i32>,
    pub point2: Vec2<i32>,
    pub color: [u8; 4],
}

impl Wall {
    pub fn new(point1: &Vec2<i32>, point2: &Vec2<i32>) -> Self {
        Wall {
            point1: point1.clone(),
            point2: point2.clone(),
            color: [0x0, 0x0, 0x0, 0x0],
        }
    }
}

pub struct Sector {
    pub wall: Vec2<i32>,
    pub height: Vec2<i32>,
    pub colors: [[u8; 4]; 2],
    pub center: Vec2<i32>,
    // Draw stuff
    pub surface_shape: [i32; consts::WIDTH as usize],
    pub surface_type: i32, //to hold points for surfaces
    pub distance: i32,     //surface index
}

impl Sector {
    pub fn new(wall: &Vec2<i32>, height: &Vec2<i32>) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            colors: [[0x0, 0x0, 0x0, 0x0], [0x0, 0x0, 0x0, 0x0]],
            center: Vec2::new(0, 0),
            surface_shape: [0; consts::WIDTH as usize],
            surface_type: 0,
            distance: 0,
        }
    }

    pub fn new_with_colors(wall: &Vec2<i32>, height: &Vec2<i32>, colors: [[u8; 4]; 2]) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            colors: colors,
            center: Vec2::new(0, 0),
            surface_shape: [0; consts::WIDTH as usize],
            surface_type: 0,
            distance: 0,
        }
    }
}

pub struct World {
    pub walls: Vec<Wall>,
    pub sectors: Vec<Sector>,
}
