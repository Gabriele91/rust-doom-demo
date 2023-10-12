#![allow(dead_code)]
// Using, d3d
use crate::math::Vec2;
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::world::{World, Material};
use crate::render::Render;
use crate::texture::TextureSet;
use crate::consts::{H_WIDTH, H_HEIGHT, WIDTH};
// Using
use std::rc::Rc;
use pixels::Pixels;

pub struct Render2D {
    pub world: Rc<World>,
    pub textures: Rc<TextureSet>,
    pub scale: f32,
    pub player_size: i32
}

impl Render2D {
    pub fn new(world: &Rc<World>, textures: &Rc<TextureSet>, scale: f32, player_size: i32) -> Self {
        Render2D {
            world: Rc::clone(&world),
            textures: Rc::clone(&textures),
            scale: scale,
            player_size: player_size
        }
    }

    pub fn draw_line(
          mut pixels: &mut Pixels
        , from: &Vec2<i32>
        , to: &Vec2<i32>
        , color: &[u8]
    ){
        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();
    
        let step_x = if to.x > from.x { 1 } else { -1 };
        let step_y = if to.y > from.y { 1 } else { -1 };
    
        let mut x = from.x;
        let mut y = from.y;
    
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
    
        while x != to.x || y != to.y {
            draw_pixel(&mut pixels, &Vec2::new(x as usize, y as usize), color);
    
            let err2 = err;
    
            if err2 > -dx {
                err -= dy;
                x += step_x;
            }
    
            if err2 < dy {
                err += dx;
                y += step_y;
            }
        }
    }
}

impl Render for Render2D {
    
    fn draw(&mut self, pixels: &mut Pixels, player: &Player) {               
        Render2D::draw_line(pixels, &Vec2::new(0, 0), &Vec2::new(WIDTH as i32, 0), &[0xFF, 0x0, 0xFF, 0xFF]);

        for sector in &self.world.sectors {
            for wall_id in sector.wall.x..sector.wall.y {
                // Wall
                let wall = &self.world.walls[wall_id as usize];
                // Wall to draw
                let mut wall_2d = [
                    wall.point1.clone(),
                    wall.point2.clone(),
                ];
                // Match material
                let wall_color = match wall.material {
                    Material::Texture(texture_map) => self.textures.set[texture_map.texture].avg_color::<4>(),
                    Material::Color(color) => color
                };
                // Player as center:
                for pwall in &mut wall_2d {
                    *pwall -= player.position.xy();
                }
                // Scale
                for pwall in &mut wall_2d {
                    *pwall = (pwall.as_vec::<f32>() * self.scale).as_vec::<i32>();
                }
                // Center of the screen
                for pwall in &mut wall_2d {
                    *pwall += Vec2::new(H_WIDTH as i32, H_HEIGHT as i32);
                }
                // Draw wall                                                        
                Render2D::draw_line(pixels, &wall_2d[0], &wall_2d[1], &wall_color);
            }
        }
        // Draw Player
        //    0
        //   /\
        // 1/_\2
        //
        let mut shape: [Vec2<i32>; 3] = [
            Vec2::new(0, self.player_size),
            Vec2::new(-self.player_size, -self.player_size),
            Vec2::new(self.player_size, -self.player_size),
        ];
        // Rotation
        for point in &mut shape {
            let x = ((point.x as f32) * player.cos() + (point.y as f32) * player.sin()) as i32;
            let y = ((point.y as f32) * player.cos() - (point.x as f32) * player.sin()) as i32;
            *point = Vec2::new(x, y);
        }
        // Scale
        for point in &mut shape {
            *point += (point.as_vec::<f32>() * self.scale).as_vec::<i32>();
        }
        // Center
        for point in &mut shape {
            *point += Vec2::new(H_WIDTH as i32, H_HEIGHT as i32);
        }      
        // Draw player
        for id in [[0,1],[1,2],[2,0]] {
            Render2D::draw_line(pixels, &shape[id[0]], &shape[id[1]], &[0x00, 0xFF, 0x00, 0xFF]);  
        }
    }

}