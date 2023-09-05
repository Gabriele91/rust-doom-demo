mod consts;
mod math;
mod world;
mod player;
mod render;
mod windows;
mod map;
mod tga;
mod texture;

// Using d3
use crate::map::Map;
use crate::player::Player;
use crate::render::Render;
use crate::texture::TextureSet;
// Using
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;
use clap::{Command, Arg, ArgMatches};
use std::rc::Rc;

fn shell_args() -> ArgMatches {
    Command::new("Rust-doom-demo")
    .author("Gabriele Di Bari")
    .version("0.0.1")
    .about("Doom-style renderer.")
    .arg(Arg::new("map")
        .short('m')
        .long("map")
        .required(true)
        .help("Map path"))
    .arg(Arg::new("textures")
        .short('t')
        .long("textures")
        .required(true)
        .help("Textures path"))
    .get_matches()
}

fn main() {
    let matches = shell_args();
    let map_path = matches.get_one::<String>("map").unwrap();
    let textures_path = matches.get_one::<String>("textures").unwrap();
    let map =  match  Map::from(map_path) {
        Some(map) => map,
        _ => panic!("Unable to load map {:?}", map_path),
    };
    let texset = match TextureSet::from(textures_path) {
        Some(texset) => Rc::new(texset),
        _ => panic!("Unable to load textures {:?}", textures_path),
    };
    
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

    // Render
    let mut render = Render::new(map.world, texset);

    // Main loop
    event_loop.run(
        move |event: Event<'_, ()>, _, control_flow: &mut ControlFlow| {
            // Get player
            let mut player: std::cell::RefMut<'_, Player> = map.player.borrow_mut();
            // Event
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
                    // Input
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
