
// Windows size
pub const RESOLUTION : u32 = 1;
pub const WIDTH: u32 = 160 * RESOLUTION;
pub const HEIGHT: u32 = 120 * RESOLUTION;

// Screen size
pub const SCREEN_RESOLUTION : u32 = 6;
pub const SCREE_WIDTH: u32 = WIDTH * SCREEN_RESOLUTION;
pub const SCREE_HEIGHT: u32 = HEIGHT * SCREEN_RESOLUTION;

// Utils
pub const H_WIDTH : u32 = WIDTH / 2;
pub const H_HEIGHT : u32 = HEIGHT / 2;
pub const BACKGROUND_COLOR: [u8; 4] = [0x22,0x22,0xff,0xff];

// Player
pub const MOVE_VELOCITY : i32 = 1;
pub const ROTATION_VELOCITY : i32 = 1;
pub const UPDOWN_VELOCITY : i32 = 1;

// Camera
pub const FOV : i32 = 180;
pub const UPDOWN_FACTOR : f32 = 64.0;

// Box
pub const BOX_DEEP: i32 = 40 * 7;
pub const BOX_HEIGHT : i32 = 40;
pub const BOX_WIDTH : i32 = 40;