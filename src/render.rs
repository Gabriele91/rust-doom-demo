// Using, d3d
use crate::consts;
use crate::math::{clamp, Vec2, Vec3};
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::world::{World, Sector};

// Using
use std::rc::Rc;
use pixels::Pixels;

#[derive(Clone, Copy, PartialEq)]
enum Face {
    Front = 0x01,
    Back = 0x02,
}

#[derive(Clone, Copy, PartialEq)]
enum SurfaceView {
    Top = 0x01,
    Bottom = 0x02,
    Mid = 0x00,
}

#[derive(Clone)]
struct WallProjector {
    wall: [Vec3<i32>; 4],
    face: Face,
    distance: i32,
    visiable: bool,
}

#[derive(Clone)]
struct Surface {
    points: Vec<i32>,
    view: SurfaceView,
}

#[derive(Clone)]
struct SectorContext {
    index: usize,
    surface: Surface,
    distance: i32,
}

pub struct Render {
    world: Rc<World>,
    sectors_context: Vec<SectorContext>
}

fn clip_behind_player(point1: &mut Vec3<i32>, point2: &Vec3<i32>) {
    let da = point1.y as f32;
    let db = point2.y as f32;
    let mut d = da - db;
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

impl WallProjector {
    
    pub fn new() -> Self {
        WallProjector {
            wall: [Vec3::new(0, 0, 0); 4],
            face: Face::Back,
            distance: 0,
            visiable: false,
        }
    }

    fn project(&mut self, player: &Player, face: &Face, wall2d: &[Vec2<i32>; 2], height: &Vec2<i32>) {
        // Set values 
        self.face = face.clone();
        // Wall direction
        let points = {
            match face {
                Face::Front => [
                    wall2d[0] - player.position.xy(),
                    wall2d[1] - player.position.xy(),
                ],
                Face::Back => [
                    wall2d[1] - player.position.xy(),
                    wall2d[0] - player.position.xy(),
                ],
            }
        };
        // Cache cos and sin
        let pcos = player.cos();
        let psin = player.sin();
        // First line in 3D
        for i in 0..2 {
            // World X
            self.wall[i].x = ((points[i].x as f32) * pcos - (points[i].y as f32) * psin) as i32;
            // World Y
            self.wall[i].y = ((points[i].y as f32) * pcos + (points[i].x as f32) * psin) as i32;
            // World Z
            self.wall[i].z = ((height.x - player.position.z) as f32
                           + ((player.updown * self.wall[i].y) as f32 / consts::UPDOWN_FACTOR))
                           as i32;

            // Second line,  X,Y are the same
            self.wall[i + 2].x = self.wall[i].x;
            self.wall[i + 2].y = self.wall[i].y;
            // Z is to be recompute with new height
            self.wall[i + 2].z = ((height.y - player.position.z) as f32
                               + ((player.updown * self.wall[i].y) as f32 / consts::UPDOWN_FACTOR))
                               as i32;
        }
        // Distance
        self.distance = distance_pw2(
            &Vec2::new(0, 0),
            &Vec2::new((self.wall[0].x + self.wall[1].x) / 2, (self.wall[0].y + self.wall[1].y) / 2),
        );
        // Clip wall behind player
        if self.wall[0].y < 1 && self.wall[1].y < 1 {
            self.visiable = false;
            return;
        }
        // Point 1 behind player, clip
        else if self.wall[0].y < 1 {
            let wall_1 = self.wall[1].clone();
            clip_behind_player(&mut self.wall[0], &wall_1); // bottom line
            let wall_3 = self.wall[3].clone();
            clip_behind_player(&mut self.wall[2], &wall_3); // top line
        }
        // Point 2 behind player, clip
        else if self.wall[1].y < 1 {
            let wall_0 = self.wall[0].clone();
            clip_behind_player(&mut self.wall[1], &wall_0); // bottom line
            let wall_2 = self.wall[2].clone();
            clip_behind_player(&mut self.wall[3], &wall_2); // top line
        }
        // Screen position
        for i in 0..4 {
            self.wall[i].x = (self.wall[i].x * consts::FOV) / self.wall[i].y + consts::H_WIDTH as i32;
            self.wall[i].y = (self.wall[i].z * consts::FOV) / self.wall[i].y + consts::H_HEIGHT as i32;
        }
        // Draw
        self.visiable = true;
    }

    fn distance_bottom_line(&self) -> i32 {
        self.wall[1].y - self.wall[0].y
    } 

    fn distance_top_line(&self) -> i32 {
        self.wall[3].y - self.wall[2].y
    } 

    fn large(&self) -> i32 {
        self.wall[1].x - self.wall[0].x
    }

}

impl Surface {
    pub fn get_surface_from_backside<'a>(&'a mut self, face: &Face, x: i32, y1: &mut i32, y2: &mut i32, colors: &[&'a [u8; 4]; 3]) -> &[u8; 4] {
        // Surface
        match face {
            Face::Back => {
                match self.view {
                    SurfaceView::Top => { 
                        *y2 = self.points[x as usize];  
                        colors[1]
                    }
                    SurfaceView::Bottom => { 
                        *y1 = self.points[x as usize];
                        colors[2] 
                    }
                    SurfaceView::Mid => {
                        colors[0]
                    }
                }
            },
            Face::Front => {
                match self.view {
                    SurfaceView::Top    => { self.points[x as usize] = *y1; } // bottom save to top
                    SurfaceView::Bottom => { self.points[x as usize] = *y2; } // top save to bottom
                    SurfaceView::Mid    => {}
                }
                colors[0]
            }
        }
    }
}

impl SectorContext {
    
    pub fn new(index: usize) -> Self {
        SectorContext {
            index: index,
            surface: Surface {
                points: vec![0; consts::WIDTH as usize],
                view: SurfaceView::Mid
            } ,
            distance: 0,
        }
    }

    pub fn start<'a>(&mut self, position: &Vec3<i32>, sector: &Sector) -> std::slice::Iter<'a, Face> {
        // Clear distance
        self.distance = 0;
        // Draw top/mid/bottom
        if position.z < sector.height.x {
            self.surface.view = SurfaceView::Top;
            self.surface.points.fill(consts::HEIGHT as i32);
            [Face::Front, Face::Back].iter()
        } else if position.z > sector.height.y {
            self.surface.view = SurfaceView::Bottom;
            self.surface.points.fill(0);
            [Face::Front, Face::Back].iter()
        } else {
            self.surface.view = SurfaceView::Mid;
            [Face::Front].iter()
        }
    }

    fn draw(&mut self,mut pixels: &mut Pixels, wall_projector: &WallProjector, colors: &[&[u8; 4]; 3]) {
        // Wall
        let wall = &wall_projector.wall;
        // y distance of bottom line
        let dyb = wall_projector.distance_bottom_line();
        // y distance of top line
        let dyt = wall_projector.distance_top_line();
        // x distance
        let mut dx = wall_projector.large();
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
            let mut y1 = ((dyb as f32 * (((x - xs) as f32 + 0.5) / (dx as f32))) + wall[0].y as f32) as i32;
            // From x1 to x, starting from closet point to current top
            let mut y2 = ((dyt as f32 * (((x - xs) as f32 + 0.5) / (dx as f32))) + wall[2].y as f32) as i32;
            // Clip Y
            y1 = clamp(y1, 0, consts::HEIGHT as i32);
            y2 = clamp(y2, 0, consts::HEIGHT as i32);
            // Update surface
            let color = self.surface.get_surface_from_backside(&wall_projector.face, x, &mut y1, &mut y2, &colors);
            // Draw wall
            for y in y1..y2 {
                draw_pixel(&mut pixels, &Vec2::new(x as usize, y as usize), color);
            }
        }
    }

}

impl Render {
    pub fn new(ref_world: Rc<World>) -> Self {
        Render {
            world: Rc::clone(&ref_world),
            sectors_context: (0..ref_world.sectors.len()).map(|i| SectorContext::new(i)).collect(),
        }
    }

    pub fn draw(&mut self, mut pixels: &mut Pixels, player: &Player) {
        // Init
        let mut wall_projector = WallProjector::new();
        // Sort
        self.sectors_context.sort_by(|left, right| right.distance.cmp(&left.distance));    
        // Mut ref to self.sectors_context
        let sectors_context = &mut self.sectors_context;
        // For each sector
        for context in sectors_context {
            // Ref to sector
            let sector = &self.world.sectors[context.index];
            // Back and front
            for face in context.start(&player.position, &sector) {
                // For each wall
                for wall_id in sector.wall.x..sector.wall.y {
                    // Wall
                    let wall = &self.world.walls[wall_id as usize];
                    // From a wall described as two points + height, to 3D world
                    wall_projector.project(&player, &face, &[wall.point1, wall.point2], &sector.height);
                    // Draw
                    if wall_projector.visiable {
                        let colors = [
                            wall.material.color_or(&[0xff; 4]),
                            sector.material[0].color_or(&[0xff; 4]),
                            sector.material[1].color_or(&[0xff; 4]),
                        ];
                        context.draw(&mut pixels, &wall_projector, &colors);
                    }
                    // Add distance
                    context.distance += wall_projector.distance;
                }
            }
            // AVG distance:
            context.distance /= sector.wall.y - sector.wall.x;
        }
    }
}
