#![allow(dead_code)]
// Using, d3d
use crate::math::Vec2;

pub struct TextureMapping {
    pub texture: usize,
    pub uv: Vec2<i32>,
    pub shade: i32
}

pub enum Material {
    Color([u8; 4]),
    Texture(TextureMapping)
}

impl Material {
    pub fn color_or<'a>(&'a self, default: &'a [u8; 4]) -> &[u8; 4] {
        match self {
            Material::Color(color) => color,
            _ => default
        }
    }
}

pub struct Wall {
    pub point1: Vec2<i32>,
    pub point2: Vec2<i32>,
    pub material: Material,
}

impl Wall {
    pub fn new(point1: &Vec2<i32>, point2: &Vec2<i32>) -> Self {
        Wall {
            point1: point1.clone(),
            point2: point2.clone(),
            material: Material::Color([0xff, 0xff, 0xff, 0xff]),
        }
    }
    pub fn new_with_material(point1: &Vec2<i32>, point2: &Vec2<i32>, material: Material) -> Self {
        Wall {
            point1: point1.clone(),
            point2: point2.clone(),
            material: material,
        }
    }
}

pub struct Sector {
    pub wall: Vec2<i32>,
    pub height: Vec2<i32>,
    pub colors: [[u8; 4]; 2],
    pub center: Vec2<i32>,
    pub material: [Material; 2]
}

impl Sector {
    pub fn new(wall: &Vec2<i32>, height: &Vec2<i32>) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            colors: [[0x0, 0x0, 0x0, 0x0], [0x0, 0x0, 0x0, 0x0]],
            center: Vec2::new(0, 0),
            material: [
                Material::Color([0xff,0xff,0xff,0xff]),
                Material::Color([0xff,0xff,0xff,0xff])
            ]
        }
    }

    pub fn new_with_colors(wall: &Vec2<i32>, height: &Vec2<i32>, colors: [[u8; 4]; 2]) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            colors: colors,
            center: Vec2::new(0, 0),
            material: [
                Material::Color([0xff,0xff,0xff,0xff]),
                Material::Color([0xff,0xff,0xff,0xff])
            ]
        }
    }
}

pub struct World {
    pub walls: Vec<Wall>,
    pub sectors: Vec<Sector>,
}
