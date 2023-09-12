#![allow(dead_code)]
// Using, d3d
use crate::math::{Vec3, self};
use crate::consts::{MOVE_VELOCITY, ROTATION_VELOCITY, UPDOWN_VELOCITY};
// Using
use winit::event::{
    Event, VirtualKeyCode
};
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Clone, Copy)]
pub struct Player {
        float_position: Vec3<f32>,
    pub position: Vec3<i32>,
    pub angle: i32,
    pub updown: i32
}

impl Player {
    pub fn new() -> Self {
        Player {
            float_position: Vec3::new(0.0,0.0,0.0),
            position: Vec3::new(0,0,0),
            angle: 0,
            updown: 0
        }
    }
    
    pub fn new_with_position(initial_position: Vec3<i32>) -> Self {
        Player {
            float_position: Vec3::new(initial_position.x as f32,initial_position.y as f32,initial_position.z as f32),
            position: initial_position,
            angle: 0,
            updown: 0
        }
    }
    
    pub fn new_with_position_angles(initial_position: Vec3<i32>, angle: i32, updown: i32) -> Self {
        Player {
            float_position: Vec3::new(initial_position.x as f32,initial_position.y as f32,initial_position.z as f32),
            position: initial_position,
            angle: angle,
            updown: updown
        }
    }

    pub fn right(&mut self) {
        self.angle = (self.angle + ROTATION_VELOCITY) % 360;
    }

    pub fn left(&mut self) {
        self.angle = ((self.angle - ROTATION_VELOCITY) + 360) % 360;
    }

    pub fn cos(&self) -> f32 {
        math::COS[self.angle as usize]
    }

    pub fn sin(&self) -> f32 {
        math::SIN[self.angle as usize]
    }

    pub fn up(&mut self) {
        self.updown -= UPDOWN_VELOCITY;
    }

    pub fn down(&mut self) {
        self.updown += UPDOWN_VELOCITY;
    }

    pub fn dirmove(&mut self, xydir: Vec3<f32>) {
        let x = self.cos() * (xydir.x as f32) + self.sin() * (xydir.y as f32);
        let y =-self.sin() * (xydir.x as f32) + self.cos() * (xydir.y as f32);
        self.translate(Vec3::new(x,y, xydir.z as f32));
    }

    pub fn translate(&mut self, direction: Vec3<f32>) {
        self.float_position += direction;
        self.position   = Vec3::new( self.float_position.x as i32, self.float_position.y as i32, self.float_position.z as i32);
    }
    
    pub fn execute_input_standard(&mut self, _event: &Event<'_, ()>,input: &WinitInputHelper) {
        if  input.key_held(VirtualKeyCode::W) 
        && !input.key_held(VirtualKeyCode::S) {
            self.translate(Vec3::new_y(MOVE_VELOCITY as f32));
        }
        if !input.key_held(VirtualKeyCode::W) 
        &&  input.key_held(VirtualKeyCode::S) {
            self.translate(Vec3::new_y(-MOVE_VELOCITY as f32));
        }        
        if  input.key_held(VirtualKeyCode::A) 
        && !input.key_held(VirtualKeyCode::D) {
            self.translate(Vec3::new_x(-MOVE_VELOCITY as f32));
        }
        if !input.key_held(VirtualKeyCode::A) 
        &&  input.key_held(VirtualKeyCode::D) {
            self.translate(Vec3::new_x(MOVE_VELOCITY as f32));
        }
        if  input.key_held(VirtualKeyCode::R) 
        && !input.key_held(VirtualKeyCode::F) {
            self.translate(Vec3::new_z(MOVE_VELOCITY as f32));
        }
        if !input.key_held(VirtualKeyCode::R) 
        &&  input.key_held(VirtualKeyCode::F) {
            self.translate(Vec3::new_z(-MOVE_VELOCITY as f32));
        }
        if  input.key_held(VirtualKeyCode::Left) 
        && !input.key_held(VirtualKeyCode::Right) {
            self.left();
        }
        if !input.key_held(VirtualKeyCode::Left) 
        &&  input.key_held(VirtualKeyCode::Right) {
            self.right();
        }
        if  input.key_held(VirtualKeyCode::Up) 
        && !input.key_held(VirtualKeyCode::Down) {
            self.up();
        }
        if !input.key_held(VirtualKeyCode::Up) 
        &&  input.key_held(VirtualKeyCode::Down) {
            self.down();
        }
    }

    pub fn execute_input_classic(&mut self, _event: &Event<'_, ()>,input: &WinitInputHelper) {
        if  input.key_held(VirtualKeyCode::W) 
        && !input.key_held(VirtualKeyCode::S) {
            self.dirmove(Vec3::new_y(MOVE_VELOCITY as f32));
        }
        if !input.key_held(VirtualKeyCode::W) 
        &&  input.key_held(VirtualKeyCode::S) {
            self.dirmove(Vec3::new_y(-MOVE_VELOCITY as f32));
        }        
        if  input.key_held(VirtualKeyCode::A) 
        && !input.key_held(VirtualKeyCode::D) {
            self.dirmove(Vec3::new_x(-MOVE_VELOCITY as f32));
        }
        if !input.key_held(VirtualKeyCode::A) 
        &&  input.key_held(VirtualKeyCode::D) {
            self.dirmove(Vec3::new_x(MOVE_VELOCITY as f32));
        }
        if  input.key_held(VirtualKeyCode::R) 
        && !input.key_held(VirtualKeyCode::F) {
            self.translate(Vec3::new_z(MOVE_VELOCITY as f32));
        }
        if !input.key_held(VirtualKeyCode::R) 
        &&  input.key_held(VirtualKeyCode::F) {
            self.translate(Vec3::new_z(-MOVE_VELOCITY as f32));
        }
        if  input.key_held(VirtualKeyCode::Left) 
        && !input.key_held(VirtualKeyCode::Right) {
            self.left();
        }
        if !input.key_held(VirtualKeyCode::Left) 
        &&  input.key_held(VirtualKeyCode::Right) {
            self.right();
        }
        if  input.key_held(VirtualKeyCode::Up) 
        && !input.key_held(VirtualKeyCode::Down) {
            self.up();
        }
        if !input.key_held(VirtualKeyCode::Up) 
        &&  input.key_held(VirtualKeyCode::Down) {
            self.down();
        }
    }
}

