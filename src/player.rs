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
    pub position: Vec3<i32>,
    pub angle: i32,
    pub updown: i32
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vec3::new(0,0,0),
            angle: 0,
            updown: 0
        }
    }
    pub fn new_with_position(initial_position: Vec3<i32>) -> Self {
        Player {
            position: initial_position,
            angle: 0,
            updown: 0
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
        self.updown += UPDOWN_VELOCITY;
    }

    pub fn down(&mut self) {
        self.updown -= UPDOWN_VELOCITY;
    }

    pub fn execute_input(&mut self, _event: &Event<'_, ()>,input: &WinitInputHelper) {
        if  input.key_held(VirtualKeyCode::W) 
        && !input.key_held(VirtualKeyCode::S) {
            self.position = self.position + Vec3::new(0,0,MOVE_VELOCITY);
        }
        if !input.key_held(VirtualKeyCode::W) 
        &&  input.key_held(VirtualKeyCode::S) {
            self.position = self.position + Vec3::new(0,0,-MOVE_VELOCITY);
        }        
        if  input.key_held(VirtualKeyCode::A) 
        && !input.key_held(VirtualKeyCode::D) {
            self.position = self.position + Vec3::new(MOVE_VELOCITY,0,0);
        }
        if !input.key_held(VirtualKeyCode::A) 
        &&  input.key_held(VirtualKeyCode::D) {
            self.position = self.position + Vec3::new(-MOVE_VELOCITY,0,0);
        }
        if  input.key_held(VirtualKeyCode::R) 
        && !input.key_held(VirtualKeyCode::F) {
            self.position = self.position + Vec3::new(0,MOVE_VELOCITY,0);
        }
        if !input.key_held(VirtualKeyCode::R) 
        &&  input.key_held(VirtualKeyCode::F) {
            self.position = self.position + Vec3::new(0,-MOVE_VELOCITY,0);
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

