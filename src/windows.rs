// Using, d3d
use crate::math::Vec2;
// Using
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, Position},
    error::OsError,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

pub fn build_windows(
    title: String,
    width: u32,
    height: u32,
    event_loop: &EventLoop<()>,
) -> Result<Window, OsError> {
    let size: LogicalSize<f64> = LogicalSize::new(width as f64, height as f64);
    WindowBuilder::new()
        .with_title(title)
        .with_inner_size(size.clone())
        .with_min_inner_size(size.clone())
        .build(&event_loop)
}

pub fn pixes_from_windows(window: &Window) -> Result<Pixels, Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(window_size.width, window_size.height, surface_texture)
}

pub fn pixes_from_size(window: &Window, width: u32, height: u32) -> Result<Pixels, Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(width, height, surface_texture)
}

pub fn clear_background(pixels: &mut Pixels, color: [u8; 4]) {
    let frame = pixels.frame_mut();
    let size = (frame.len() / color.len()) as usize;
    frame.copy_from_slice(&color.repeat(size));
}

pub fn draw_pixel<const CHANNELS: usize>(
    pixels: &mut Pixels,
    position: &Vec2<usize>,
    color: [u8; CHANNELS],
) {
    let size = pixels.texture().size();
    if position.x >= size.width as usize|| position.y >= size.height as usize {
        return;
    }
    let frame = pixels.frame_mut();
    let row_size = (size.width as usize) * CHANNELS; // 4 colors per byte
    let offset: usize = position.y * row_size + position.x * CHANNELS;
    let mut ptr = frame.as_mut_ptr();
    unsafe {
        ptr = ptr.add(offset);
        for i in 0..CHANNELS {
            (*ptr) = color[i];
            ptr = ptr.add(1);
        }
    }
}
