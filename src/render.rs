// Using, d3d
use crate::consts;
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::math::{Vec2, Vec3, clamp};

// Using
use pixels::Pixels;

fn draw_wall(mut pixels: &mut Pixels, wall : &[Vec3<i32>; 4]) {
    // y distance of bottom line
    let dyb = wall[1].y - wall[0].y;
    // y distance of top line
    let dyt = wall[3].y - wall[2].y;
    // x distance
    let mut dx = wall[1].x - wall[0].x;
    if dx == 0 {
        dx = 1;
    }
    // Clip X
    let x1 = clamp(wall[0].x, 0, consts::WIDTH as i32);
    let x2 = clamp(wall[1].x, 0, consts::WIDTH as i32);
    // Draw line
    for x in x1..x2 {
        // From x1 to x, starting from closet point to current bottom 
        let mut y1 = ((dyb as f32 * (((x - x1) as f32 + 0.5) / (dx as f32))) + wall[0].y as f32) as i32;
        // From x1 to x, starting from closet point to current top 
        let mut y2 = ((dyt as f32 * (((x - x1) as f32 + 0.5) / (dx as f32))) + wall[2].y as f32) as i32;
        // Clip Y
        y1 = clamp(y1, 0, consts::HEIGHT as i32);
        y2 = clamp(y2, 0, consts::HEIGHT as i32);
        // Draw
        for y in y1..y2 {
            draw_pixel(
                &mut pixels,
                &Vec2::new(x as usize, y as usize),
                [0xff, 0xff, 0x0, 0xff],
            );
        }
    }
}

fn clip_behind_player(point1: &mut Vec3<i32>, point2: &Vec3<i32>) {
    let da = point1.y as f32;
    let db = point1.y as f32;
    let mut d = db - da;
    if d == 0.0 {
        d = 1.0;
    }
    let s = da / d;
    point1.x += (s * (point2.x - point1.x) as f32) as i32;
    point1.y += (s * (point2.y - point1.y) as f32) as i32;
    point1.z += (s * (point2.z - point1.z) as f32) as i32;
    if point1.y == 0 {
        point1.y = 1;
    }
}

fn project_wall(player: &Player, wall : &mut [Vec3<i32>; 4], points: &[Vec2<i32>; 2]) {
    // First line in 3D
    for i in 0..2 {
        // World X
        wall[i].x = ((points[i].x as f32) * player.cos() - (points[i].y as f32) * player.sin()) as i32;
        // World Y
        wall[i].y = ((points[i].y as f32) * player.cos() + (points[i].x as f32) * player.sin()) as i32;
        // World Z
        wall[i].z = (0.0 - player.position.z as f32 + ((player.updown * wall[i].y) as f32 / consts::UPDOWN_FACTOR)) as i32;
    }
    // Second line,  X,Y are the same, Z is the same + HEIGHT
    for i in 2..4 {
        wall[i].x = wall[i-2].x;
        wall[i].y = wall[i-2].y;
        wall[i].z = wall[i-2].z + consts::BOX_HEIGHT;
    }
    // Clip wall behind player
    if wall[0].y < 1 && wall[1].y < 1 {
        return;
    // Point 1 behind player, clip
    } else if wall[0].y  < 1 {
        let wall_1 = wall[1];
        clip_behind_player(&mut wall[0], &wall_1); // bottom line
        let wall_3 = wall[3];
        clip_behind_player(&mut wall[2], &wall_3); // top line                        
    // Point 2 behind player, clip
    } else if wall[1].y  < 1 {
        let wall_0 = wall[0];
        clip_behind_player(&mut wall[1], &wall_0); // bottom line
        let wall_2 = wall[2];
        clip_behind_player(&mut wall[3], &wall_2); // top line     
    }
    // Screen position
    for i in 0..4 {
        wall[i].x = (wall[i].x * consts::FOV) / wall[i].y + consts::H_WIDTH as i32;
        wall[i].y = (wall[i].z * consts::FOV) / wall[i].y + consts::H_HEIGHT as i32;
    }
}

pub fn draw_3d(mut pixels: &mut Pixels, player: &Player) {
    // Wall points
    let mut wall = [
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, 0),
    ];
    // Wall description
    let mut points = [
        Vec2::new(-player.position.x + consts::BOX_WIDTH, -player.position.y + 10),
        Vec2::new(-player.position.x + consts::BOX_WIDTH, -player.position.y + consts::BOX_DEEP + 10),
    ];
    // From a wall described as two points + HEIGHT, to 3D world
    project_wall(&player, &mut wall, &points);
    // Draw
    draw_wall(&mut pixels, &wall);
}
