// Using, d3d
use crate::consts;
use crate::math::{clamp, Vec2, Vec3};
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::world::{Sector, Wall, Wolrd};

// Using
use pixels::Pixels;

enum Face {
    Front = 0x01,
    Back  = 0x02
}

fn clip_behind_player(point1: &mut Vec3<i32>, point2: &Vec3<i32>) {
    let da = point1.y as f32;
    let db = point2.y as f32;
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

fn distance_pw2(point1: &Vec2<i32>, point2: &Vec2<i32>) -> i32 {
    let delta = *point1 - *point2;
    let delta_pw2 = delta * delta;
    return delta_pw2.x + delta_pw2.y;
}

fn project_wall(
    player: &Player,
    wall: &mut [Vec3<i32>; 4],
    points: &[Vec2<i32>; 2],
    height: &Vec2<i32>,
) -> (bool, i32) {
    // Cache cos and sin
    let pcos = player.cos();
    let psin = player.sin();
    // First line in 3D
    for i in 0..2 {
        // World X
        wall[i].x = ((points[i].x as f32) * pcos - (points[i].y as f32) * psin) as i32;
        // World Y
        wall[i].y = ((points[i].y as f32) * pcos + (points[i].x as f32) * psin) as i32;
        // World Z
        wall[i].z = ((height.x - player.position.z) as f32
                  + ((player.updown * wall[i].y) as f32 / consts::UPDOWN_FACTOR)) as i32;

        // Second line,  X,Y are the same
        wall[i + 2].x = wall[i].x;
        wall[i + 2].y = wall[i].y;
        // Z is the same + height
        wall[i + 2].z = wall[i].z + height.y;
    }
    // Distance
    let wall_distance_pw2 = distance_pw2(
        &Vec2::new(0, 0),
        &Vec2::new((wall[0].x + wall[1].x) / 2, (wall[0].y + wall[1].y) / 2),
    );
    // Clip wall behind player
    if wall[0].y < 1 && wall[1].y < 1 {
        // No draw
        return (false, wall_distance_pw2);
    }  
    // Point 1 behind player, clip
    else if wall[0].y < 1 {
        let wall_1 = wall[1].clone();
        clip_behind_player(&mut wall[0], &wall_1); // bottom line
        let wall_3 = wall[3].clone();
        clip_behind_player(&mut wall[2], &wall_3); // top line
    }  
    // Point 2 behind player, clip
    else if wall[1].y < 1 {
        let wall_0 = wall[0].clone();
        clip_behind_player(&mut wall[1], &wall_0); // bottom line
        let wall_2 = wall[2].clone();
        clip_behind_player(&mut wall[3], &wall_2); // top line
    }
    // Screen position
    for i in 0..4 {
        wall[i].x = (wall[i].x * consts::FOV) / wall[i].y + consts::H_WIDTH as i32;
        wall[i].y = (wall[i].z * consts::FOV) / wall[i].y + consts::H_HEIGHT as i32;
    }
    // Draw
    return (true, wall_distance_pw2);
}

fn draw_wall(mut pixels: &mut Pixels, wall: &[Vec3<i32>; 4], color: &[u8; 4]) {
    // y distance of bottom line
    let dyb = wall[1].y - wall[0].y;
    // y distance of top line
    let dyt = wall[3].y - wall[2].y;
    // x distance
    let mut dx = wall[1].x - wall[0].x;
    if dx == 0 {
        dx = 1;
    }
    // Hold initial x1 starting position 
    let xs = wall[0].x; 
    // Clip X
    let x1 = clamp(wall[0].x, 0, consts::WIDTH as i32);
    let x2 = clamp(wall[1].x, 0, consts::WIDTH as i32);
    // Draw line
    for x in x1..x2 {
        // From x1 to x, starting from closet point to current bottom
        let mut y1 =
            ((dyb as f32 * (((x - xs) as f32 + 0.5) / (dx as f32))) + wall[0].y as f32) as i32;
        // From x1 to x, starting from closet point to current top
        let mut y2 =
            ((dyt as f32 * (((x - xs) as f32 + 0.5) / (dx as f32))) + wall[2].y as f32) as i32;
        // Clip Y
        y1 = clamp(y1, 0, consts::HEIGHT as i32);
        y2 = clamp(y2, 0, consts::HEIGHT as i32);
        // Draw
        for y in y1..y2 {
            draw_pixel(&mut pixels, &Vec2::new(x as usize, y as usize), *color);
        }
    }
}

pub fn draw_3d(mut pixels: &mut Pixels, player: &Player, wolrd: &mut Wolrd) {
    // Wall points
    let mut projected_wall = [
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, 0),
        Vec3::new(0, 0, 0),
    ];
    // Sort
    wolrd
        .sectors
        .sort_by(|left, right| right.distance.cmp(&left.distance) );
    // For each sector
    for sector in &mut wolrd.sectors {
        // Clear distance
        sector.distance = 0;
        // Back and front
        for face in &[Face::Back, Face::Front]{ 
            // For each wall
            for wall in sector.wall.x..sector.wall.y {
                // Wall description
                let points = { 
                    match face {
                        Face::Front => [
                            wolrd.walls[wall as usize].point1 - player.position.xy(),
                            wolrd.walls[wall as usize].point2 - player.position.xy(),
                        ],
                        Face::Back => [
                            wolrd.walls[wall as usize].point2 - player.position.xy(),
                            wolrd.walls[wall as usize].point1 - player.position.xy(),
                        ]
                    }
                };
                // From a wall described as two points + height, to 3D world
                let (draw, distance) =
                    project_wall(&player, &mut projected_wall, &points, &sector.height);
                // Draw
                if draw {
                    draw_wall(
                        &mut pixels,
                        &projected_wall,
                        &wolrd.walls[wall as usize].color,
                    );
                }
                // Add distance
                sector.distance += distance as i32;
            }
        }
        // AVG distance:
        sector.distance /= sector.wall.y - sector.wall.x;
    }
}
