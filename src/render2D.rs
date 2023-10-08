#![allow(dead_code)]
// Using, d3d
use crate::math::Vec2;
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::world::World;
use crate::render::Render;
// Using
use std::rc::Rc;
use pixels::Pixels;

pub struct Render2D {
    pub world: Rc<World>,
}

impl Render2D {
    pub fn new(world: &Rc<World>) -> Self {
        Render2D {
            world: Rc::clone(&world),
        }
    }

    pub fn draw_line(
          mut pixels: &mut Pixels
        , from: &Vec2<i32>
        , to: &Vec2<i32>
        , color: &[u8]
    ){
        for y in from.y..to.y {
            for x in from.x..to.x {
                draw_pixel(
                    &mut pixels, 
                    &Vec2::new(x as usize,y as usize), 
                    &color
                );
            }
        }
    }
}

impl Render for Render2D {
    
    fn draw(&mut self, mut pixels: &mut Pixels, player: &Player) {
        for sector in &self.world.sectors {
            for wall_id in sector.wall.x..sector.wall.y {
                let from = &self.world.walls[wall_id as usize].point1;
                let to = &self.world.walls[wall_id as usize].point2;
                Render2D::draw_line(pixels, &from, &to, &[0x00, 0x00, 0xFF]);
            }
        }
    }

}