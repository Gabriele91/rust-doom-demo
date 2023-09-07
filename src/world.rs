#![allow(dead_code)]
// Using, d3d
use crate::math::Vec2;

#[derive(Clone, Copy)]
pub struct TextureMapping {
    pub texture: usize,
    pub uv: Vec2<i32>,
    pub shade: u8
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct SectorHeight {
    pub top: i32,
    pub bottom: i32 
}

impl SectorHeight {

    pub fn new(top: i32, bottom: i32) -> Self {
        SectorHeight {
            top: top,
            bottom: bottom
        }
    }

    pub fn height(&self) -> i32 {
        self.top - self.bottom
    }

    pub fn center(&self) -> f32 {
        (self.top + self.bottom) as f32 / 2.0
    }
}

pub struct Sector {
    pub wall: Vec2<i32>,
    pub height: SectorHeight,
    pub material: [Material; 2]
}

impl Sector {

    pub fn new(wall: &Vec2<i32>, height: &SectorHeight) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            material: [
                Material::Color([0xff,0xff,0xff,0xff]),
                Material::Color([0xff,0xff,0xff,0xff])
            ]
        }
    }

    pub fn new_with_colors(wall: &Vec2<i32>, height: &SectorHeight, colors: [[u8; 4]; 2]) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            material: [
                Material::Color(colors[0]),
                Material::Color(colors[1])
            ]
        }
    }

    pub fn new_with_material(wall: &Vec2<i32>, height: &SectorHeight, material: Material) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            material: [
                material.clone(),
                material.clone(),
            ]
        }
    }

    pub fn new_with_materials(wall: &Vec2<i32>, height: &SectorHeight, materials: [Material; 2]) -> Self {
        Sector {
            wall: wall.clone(),
            height: height.clone(),
            material: materials
        }
    }
}

pub struct World {
    pub walls: Vec<Wall>,
    pub sectors: Vec<Sector>,
}
