#![allow(dead_code)]
use pixels::Pixels;
use crate::windows::draw_pixel;

// Using, d3d
use crate::math::Vec2;
use crate::tga::decode_tga;
// Using
use std::fs;

pub struct Texture {
    pub dimensions: Vec2<usize>,
    pub channels: u8,
    pub data: Vec<u8>,
}

impl Texture {
    
    pub fn row_size(&self) -> usize {
        self.dimensions.x * (self.channels as usize)
    }

    pub fn pixel_index(&self, x: usize, y: usize) -> usize {
        return  y * self.row_size() + x * (self.channels as usize);
    }

    pub fn fix_pixel<const CHANNELS: usize>(&self, x: usize, y: usize) -> [u8; CHANNELS] {
        let mut colors:[u8; CHANNELS] =[0xff; CHANNELS];
        let pindex = self.pixel_index(x, y);
        for c in 0..self.channels as usize {
            colors[c] = self.data[pindex+c];
        }
        return colors;
    }

    pub fn pixel(&self, x: usize, y: usize) -> &[u8] {
        let index = self.pixel_index(x, y);
        let end_index = index + self.channels as usize;
        &self.data[index..end_index]
    }

    pub fn draw(&self, mut pixels: &mut Pixels) {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                draw_pixel(&mut pixels, &Vec2::new(x, y), self.pixel(x, y));
            }
        }
    }

}

pub struct TextureSet {
    pub set: Vec<Texture>,
}

impl TextureSet {

    fn new() -> Self {
        TextureSet { 
            set: vec![] 
        }
    }

    pub fn from(path: &str) -> Option<Self> {
        if let Ok(entries) = fs::read_dir(path) {
            // Create set
            let mut textures = TextureSet::new();
            // Read all
            for res_entry in entries {
                if let Ok(entry) = res_entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "tga" {
                            let raw_data = fs::read(&path).unwrap();
                            let mut new_texture = Texture {
                                dimensions: Vec2 { x: 0, y: 0 },
                                channels: 0,
                                data: vec![],
                            };
                            let mut format: u8 = 0;
                            let mut colors: u8 = 0;
                            if decode_tga(
                                &mut new_texture.data,
                                &mut new_texture.dimensions.x,
                                &mut new_texture.dimensions.y,
                                &mut format,
                                &mut colors,
                                &raw_data.as_slice(),
                            ) {

                                match format {
                                    1 | 3 | 4 => { 
                                        new_texture.channels = format; 
                                        textures.set.push(new_texture) 
                                    },
                                    2 => println!("{:?} does not supported", &path),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            // Return
            return Some(textures);
        }
        return None;
    }
}
