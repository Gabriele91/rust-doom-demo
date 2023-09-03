mod consts;
mod math;
mod player;
mod render;
mod windows;
// Using d3d
use crate::player::Player;
use pixels::Pixels;
// Using
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;
fn main() {
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

    // Main loop
    event_loop.run(
        move |event: Event<'_, ()>, _, control_flow: &mut ControlFlow| {
            match event {
                // Winit_input_helper doesn't support this event
                Event::RedrawRequested(_) => {
                    windows::clear_background(&mut pixels, consts::BACKGROUND_COLOR);
                    render::draw_3d(&mut pixels, &player);
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
