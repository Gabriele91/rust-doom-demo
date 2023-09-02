mod consts;
mod math;
mod player;
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

fn draw_wall(
    mut pixels: &mut Pixels,
    mut x1: i32,
    mut x2: i32,
    b1: i32,
    b2: i32,
    t1: i32,
    t2: i32,
) {
    // y distance of bottom line
    let dyb = b2 - b1;
    // y distance of top line
    let dyt = t2 - t1;
    // x distance
    let mut dx = x2 - x1;
    if dx == 0 {
        dx = 1;
    }
    // Clip X
    x1 = math::clamp(x1, 0, consts::WIDTH as i32);
    x2 = math::clamp(x2, 0, consts::WIDTH as i32);
    // Hold inizial x1
    let xs = x1;
    // Draw line
    for x in x1..x2 {
        let mut y1 = ((dyb as f32 * (((x - xs) as f32 + 0.5) / (dx as f32))) + b1 as f32) as i32;
        let mut y2 = ((dyt as f32 * (((x - xs) as f32 + 0.5) / (dx as f32))) + t1 as f32) as i32;
        // Clip Y
        y1 = math::clamp(y1, 0, consts::HEIGHT as i32);
        y2 = math::clamp(y2, 0, consts::HEIGHT as i32);
        // Draw
        for y in y1..y2 {
            windows::draw_pixel(
                &mut pixels,
                &math::Vec2::new(x as usize, y as usize),
                [0xff, 0xff, 0x0, 0xff],
            );
        }
    }
}

fn clip_behind_player(x1: &mut i32, y1: &mut i32, z1: &mut i32, x2: i32, y2: i32, z2: i32) {
    let da = *y1 as f32;
    let db = y2 as f32;
    let mut d = db - da;
    if d == 0.0 {
        d = 1.0;
    }
    let s = da / d;
    *x1 = *x1 + (s * (x2-*x1) as f32) as i32;
    *y1 = *y1 + (s * (y2-*y1) as f32) as i32;
    *z1 = *z1 + (s * (z2-*z1) as f32) as i32;
    if *y1 == 0 {
        *y1 = 1;
    }
}

fn draw_3d(mut pixels: &mut Pixels, player: &Player) {
    const RESPRESERVE: f32 = 64.0;
    // Matrix
    let mut wx: [i32; 4] = [0, 0, 0, 0];
    let mut wy: [i32; 4] = [0, 0, 0, 0];
    let mut wz: [i32; 4] = [0, 0, 0, 0];
    // Points
    let x1 = -player.position.x + consts::BOX_WIDTH;
    let y1 = -player.position.y + 10;
    let x2 = -player.position.x + consts::BOX_WIDTH;
    let y2 = -player.position.y + consts::BOX_DEEP + 10;
    // World X
    wx[0] = ((x1 as f32) * player.cos() - (y1 as f32) * player.sin()) as i32;
    wx[1] = ((x2 as f32) * player.cos() - (y2 as f32) * player.sin()) as i32;
    wx[2] = wx[0];
    wx[3] = wx[1];
    // World Y
    wy[0] = ((y1 as f32) * player.cos() + (x1 as f32) * player.sin()) as i32;
    wy[1] = ((y2 as f32) * player.cos() + (x2 as f32) * player.sin()) as i32;
    wy[2] = wy[0];
    wy[3] = wy[1];
    // World Z
    wz[0] =
        (0.0 - player.position.z as f32 + ((player.updown * wy[0]) as f32 / RESPRESERVE)) as i32;
    wz[1] =
        (0.0 - player.position.z as f32 + ((player.updown * wy[1]) as f32 / RESPRESERVE)) as i32;
    wz[2] = wz[0] + consts::BOX_HEIGHT;
    wz[3] = wz[1] + consts::BOX_HEIGHT;
    // Wall behind player
    if wy[0] < 1 && wy[1] < 1 {
        return;
    // Point 1 behind player, clip
    } else if wy[0] < 1 {
        let (wx1, wy1, wz1) = (wx[1], wy[1], wz[1]);
        clip_behind_player(&mut wx[0], &mut wy[0], &mut wz[0], wx1, wy1, wz1); // bottom line
        let (wx3, wy3, wz3) = (wx[3], wy[3], wz[3]);
        clip_behind_player(&mut wx[2], &mut wy[2], &mut wz[2], wx3, wy3, wz3); // top line
    // Point 2 behind player, clip
    } else if wy[1] < 1 {
        let (wx0, wy0, wz0) = (wx[0], wy[0], wz[0]);
        clip_behind_player(&mut wx[1], &mut wy[1], &mut wz[1], wx0, wy0, wz0); // bottom line
        let (wx2, wy2, wz2) = (wx[2], wy[2], wz[2]);
        clip_behind_player(&mut wx[3], &mut wy[3], &mut wz[3], wx2, wy2, wz2); // top line
    }
    // Screen position
    for i in 0..4 {
        wx[i] = (wx[i] * consts::FOV) / wy[i] + consts::H_WIDTH as i32;
        wy[i] = (wz[i] * consts::FOV) / wy[i] + consts::H_HEIGHT as i32;
    }
    // Draw
    draw_wall(&mut pixels, wx[0], wx[1], wy[0], wy[1], wy[2], wy[3]);
}

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
                    draw_3d(&mut pixels, &player);
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
