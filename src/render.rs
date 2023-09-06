// Using, d3d
use crate::consts;
use crate::math::{clamp, Vec2, Vec3};
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::world::{World, Sector, Material, TextureMapping};
use crate::texture::TextureSet;
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
    wall_offset: i32,
}

#[derive(Clone)]
struct SectorContext {
    index: usize,
    surface: Surface,
    distance: i32,
}

pub struct Render {
    pub world: Rc<World>,
    pub textures: Rc<TextureSet>,
    sectors_context: Vec<SectorContext>
}

fn clip_behind_player(point1: &mut Vec3<i32>, point2: &Vec3<i32>) {
    let da = point1.y as f32;
    let db = point2.y as f32;
    let d = da - db;
    if d == 0.0 {
        if (*point1).y == 0 { (*point1).y = 1; }
    } else {
        let s = da / d;
        let dv3 = *point2 - *point1;
        (*point1).x = ((*point1).x as f32 + (s * (dv3.x as f32))) as i32;
        (*point1).y = ((*point1).y as f32 + (s * (dv3.y as f32))) as i32;
        (*point1).z = ((*point1).z as f32 + (s * (dv3.z as f32))) as i32;
        if (*point1).y == 0 { (*point1).y = 1; }
    }
}

fn distance(point1: &Vec2<i32>, point2: &Vec2<i32>) -> i32 {
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
        self.distance = distance(
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

    fn look_and_move_updown(&self, player: &Player) -> (f32, f32) {
        // Looks up and down factor
        let factor = (consts::FOV as f32) / (consts::UPDOWN_FACTOR as f32) - 0.1;
        // Start
        let look_updown = -(player.updown as f32) * factor;    
        // Move
        let mut move_updown = (player.position.z - self.wall_offset) as f32 / (consts::H_HEIGHT as f32); 
        if move_updown == 0.0 { move_updown = 0.001; }
        // Return 
        return (look_updown, move_updown);
    }

    fn plane_uv(&self, player: &Player, mut x:i32 , mut y:i32 , look_updown: f32, move_updown: f32, tile: f32) -> (f32, f32) {
        let xo = consts::H_WIDTH as i32;
        let yo = consts::H_HEIGHT as i32;
        x -= xo;
        y -= yo;
        let mut z = y as f32 + look_updown; if z == 0.0 { z = 0.0001; }
        let fx = (x as f32) / z * move_updown * tile;
        let fy = (consts::FOV as f32) / z * move_updown * tile;
        let mut rx = fx * player.sin() - fy * player.cos() + ((player.position.y as f32)/(yo as f32) * tile); 
        let mut ry: f32 = fx * player.cos() + fy * player.sin() - ((player.position.x as f32)/(yo as f32) * tile); 
        if rx < 0.0 { rx=-rx+1.0; }
        if ry < 0.0 { ry=-ry+1.0; }
        return (rx,ry);
    }

    pub fn draw<'a>(
        &'a mut self, 
        mut pixels: &mut Pixels,
        player: &Player,
        face: &Face, 
        x: i32, 
        u: f32,
        mut y1: i32, 
        mut y2: i32, 
        mut v: f32,
        vs: f32,
        textures: &TextureSet,
        materials: &[&Material; 3]
    ) {
        // Material
        let mut material = materials[0];
        // Surface
        match face {
            Face::Back => {
                // Cases
                match self.view {
                    SurfaceView::Top => { 
                        y2 = self.points[x as usize];  
                        material = materials[1];
                    },
                    SurfaceView::Bottom => { 
                        y1 = self.points[x as usize];
                        material = materials[2];
                    },
                    SurfaceView::Mid => {
                        return;
                    },
                }
                // tiling
                let tile = match material {
                     Material::Texture(map) => {
                        map.uv.x as f32 
                        * textures.set[map.texture].dimensions.x as f32 
                        * (consts::RESOLUTION as f32) 
                        * consts::PLANE_TILE_FACTOR
                    },
                     _ => 1.0
                };
                // Get look updown
                let (look_updown, move_updown) = self.look_and_move_updown(&player);
                // Draw
                for y in y1..y2 {
                    // Plane uv
                    let (pu, pv) = self.plane_uv(&player, x, y, look_updown, move_updown, tile);
                    // Get color
                    let colors = match material {
                        Material::Texture(map) => {
                            let colors_slice = &textures.set[map.texture].uv_pixel_shade(pu, pv, map.shade);
                            [colors_slice[0],colors_slice[1],colors_slice[2],colors_slice[3]]
                        },
                        Material::Color(color) => *color
                    };
                    draw_pixel(&mut pixels, &Vec2::new(x as usize, y as usize), &colors);
                }
            },
            Face::Front => {
                // Cases
                match self.view {
                    SurfaceView::Top    => { self.points[x as usize] = y1; }, // bottom save to top
                    SurfaceView::Bottom => { self.points[x as usize] = y2; }, // top save to bottom
                    SurfaceView::Mid    => {  },
                }
                for y in y1..y2 {
                    let colors = match material {
                        Material::Texture(map) => {
                            let colors_slice = &textures.set[map.texture].uv_pixel_shade(u, v, map.shade);
                            [colors_slice[0],colors_slice[1],colors_slice[2],colors_slice[3]]
                        },
                        Material::Color(color) => *color
                    };
                    draw_pixel(&mut pixels, &Vec2::new(x as usize, y as usize), &colors);
                    v += vs;
                }
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
                view: SurfaceView::Mid,
                wall_offset: 0
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
            self.surface.wall_offset = sector.height.x;
            self.surface.points.fill(consts::HEIGHT as i32);
            [Face::Front, Face::Back].iter()
        } else if position.z > sector.height.y {
            self.surface.view = SurfaceView::Bottom;
            self.surface.wall_offset = sector.height.y;
            self.surface.points.fill(0);
            [Face::Front, Face::Back].iter()
        } else {
            self.surface.view = SurfaceView::Mid;
            [Face::Front].iter()
        }
    }

    fn u_texturing(&self, textures: &TextureSet, x1: i32, x2:i32, map: &TextureMapping) -> (f32, f32){
        let step =(textures.set[map.texture].dimensions.x as i32 * map.uv.x) as f32 / ((x2-x1) as f32);
        let start: f32 = if x1 < 0 { -step * (x1 as f32) } else { 0.0 };
        return (start,step);
    }

    fn v_texturing(&self, textures: &TextureSet, y1: i32, y2:i32, map: &TextureMapping) -> (f32, f32){
        let step = (textures.set[map.texture].dimensions.y as i32 * map.uv.y) as f32 / ((y2-y1) as f32);
        let start: f32 = if y1 < 0 { -step * y1 as f32 } else { 0.0 };
        return (start,step);
    }

    fn draw(
        &mut self, 
        mut pixels: &mut Pixels, 
        wall_projector: &WallProjector, 
        player: &Player,
        textures: &TextureSet,
        materials: &[&Material; 3]
    ) {
        // Wall
        let wall = &wall_projector.wall;
        // y distance of bottom line
        let dyb = wall_projector.distance_bottom_line();
        // y distance of top line
        let dyt = wall_projector.distance_top_line();
        // x distance
        let dx: i32 = wall_projector.large(); if dx == 0 { return; }
        // Hold initial x1 starting position
        let xs = wall[0].x;
        // Texture U
        let (mut u_coord, u_step) = match materials[0] {
            Material::Texture(map) => self.u_texturing(textures, wall[0].x, wall[1].x, map),
            _ => (0.0,0.0)
        };
        // Clip X
        let x1 = clamp(wall[0].x, 0, consts::WIDTH as i32);
        let x2 = clamp(wall[1].x, 0, consts::WIDTH as i32);
        // Draw line
        for x in x1..x2 {
            // From x1 to x, starting from closet point to current bottom
            let mut y1: i32 = dyb * (((x - xs) as f32 + 0.5) as i32) / dx + wall[0].y;
            // From x1 to x, starting from closet point to current top
            let mut y2: i32 = dyt * (((x - xs) as f32 + 0.5) as i32) / dx + wall[2].y;
            // texture: i32 V
            let (v_coord, v_step)= match materials[0] {
                Material::Texture(map) => self.v_texturing(textures, y1, y2, map),
                _ => (0.0,0.0)
            }; 
            // Clip Y
            y1 = clamp(y1, 0, consts::HEIGHT as i32);
            y2 = clamp(y2, 0, consts::HEIGHT as i32);
            // Draw
            self.surface.draw(
                &mut pixels,
                &player,
                &wall_projector.face, 
                x, u_coord, 
                y1, y2, v_coord, v_step, 
                textures, &materials
            );
            // Next
            u_coord += u_step;
        }
    }

}

impl Render {
    pub fn new(world: Rc<World>, textures: Rc<TextureSet>) -> Self {
        Render {
            world: Rc::clone(&world),
            textures: Rc::clone(&textures),
            sectors_context: (0..world.sectors.len()).map(|i| SectorContext::new(i)).collect(),
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
                        let materials = [
                            &wall.material,
                            &sector.material[0],
                            &sector.material[1],
                        ];
                        context.draw(
                            &mut pixels,
                            &wall_projector, 
                            &player,
                            self.textures.as_ref(),
                            &materials
                        );
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
