mod consts;
mod math;
mod world;
mod player;
mod render;
mod windows;
// Using d3d
use crate::math::Vec2;
use crate::player::Player;
use crate::world::{World, Sector, Wall};
use crate::render::{Render};
// Using
use std::rc::Rc;
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

fn main() {

    let example = Rc::new(World {
        walls: vec![
            Wall { point1: Vec2::new(0, 0)  , point2: Vec2::new(32, 0) , color: [0xff, 0xff, 0xff, 0xff] },
            Wall { point1: Vec2::new(32, 0) , point2: Vec2::new(32, 32), color: [0xCC, 0xCC, 0xCC, 0xff] },
            Wall { point1: Vec2::new(32, 32), point2: Vec2::new(0, 32) , color: [0xff, 0xff, 0xff, 0xff] },
            Wall { point1: Vec2::new(0, 32) , point2: Vec2::new(0, 0)  , color: [0xCC, 0xCC, 0xCC, 0xff] },

            Wall { point1: Vec2::new(64, 0) , point2: Vec2::new(96, 0) , color: [0x00, 0xff, 0xff, 0xff] },
            Wall { point1: Vec2::new(96, 0) , point2: Vec2::new(96, 32), color: [0x00, 0xCC, 0xCC, 0xff] },
            Wall { point1: Vec2::new(96, 32), point2: Vec2::new(64, 32), color: [0x00, 0xff, 0xff, 0xff] },
            Wall { point1: Vec2::new(64, 32), point2: Vec2::new(64, 0) , color: [0x00, 0xCC, 0xCC, 0xff] },

            Wall { point1: Vec2::new(64, 64), point2: Vec2::new(96, 64), color: [0xff, 0x00, 0xff, 0xff] },
            Wall { point1: Vec2::new(96, 64), point2: Vec2::new(96, 96), color: [0xCC, 0x00, 0xCC, 0xff] },
            Wall { point1: Vec2::new(96, 96), point2: Vec2::new(64, 96), color: [0xff, 0x00, 0xff, 0xff] },
            Wall { point1: Vec2::new(64, 96), point2: Vec2::new(64, 64), color: [0xCC, 0x00, 0xCC, 0xff] },

            Wall { point1: Vec2::new(0, 64) , point2: Vec2::new(32, 64), color: [0xff, 0xff, 0x00, 0xff] },
            Wall { point1: Vec2::new(32, 64), point2: Vec2::new(32, 96), color: [0xCC, 0xCC, 0x00, 0xff] },
            Wall { point1: Vec2::new(32, 96), point2: Vec2::new(0, 96) , color: [0xff, 0xff, 0x00, 0xff] },
            Wall { point1: Vec2::new(0, 96) , point2: Vec2::new(0, 64) , color: [0xCC, 0xCC, 0x00, 0xff] },
        ],
        sectors: vec![
            Sector::new_with_colors(&Vec2::new(0, 4),  &Vec2::new(0, 40), [[0xff,0x0, 0x00,0xff], [0x00,0xff,0x00,0xff]]),
            Sector::new_with_colors(&Vec2::new(4, 8),  &Vec2::new(0, 40), [[0x00,0xff,0x00,0xff], [0xff,0xff,0x00,0xff]]),
            Sector::new_with_colors(&Vec2::new(8, 12), &Vec2::new(0, 40), [[0xff,0x00,0xff,0xff], [0x00,0xff,0xff,0xff]]),
            Sector::new_with_colors(&Vec2::new(12, 16),&Vec2::new(0, 40), [[0xff,0xff,0x00,0xff], [0xff,0x00,0xff,0xff]]),
        ],
    });
    // Draw
    {
        // Inputs
        let mut input: WinitInputHelper = WinitInputHelper::new();
        let event_loop = EventLoop::new();

        // Window
        let window = windows::build_windows(
            String::from("Doom style engine"),
            consts::SCREE_WIDTH,
            consts::SCREE_HEIGHT,
            &event_loop,
        )
        .unwrap();

        // Surface
        let mut pixels = windows::pixes_from_size(&window, consts::WIDTH, consts::HEIGHT).unwrap();

        // Logic
        let mut player = Player::new_with_position(math::Vec3::new(70, -110, 20));

        // Render
        let mut render = Render::new(example);

        // Main loop
        event_loop.run(
            move |event: Event<'_, ()>, _, control_flow: &mut ControlFlow| {
                match event {
                    // Winit_input_helper doesn't support this event
                    Event::RedrawRequested(_) => {
                        windows::clear_background(&mut pixels, consts::BACKGROUND_COLOR);
                        render.draw(&mut pixels, &player);
                        if let Err(_) = pixels.render() {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                    }
                    _ => {
                        if input.update(&event) {
                            // Close events
                            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                            // Player
                            player.execute_input(&event, &input);
                            // Draw
                            window.request_redraw();
                        }
                    }
                }

            },
        );
    }
}
