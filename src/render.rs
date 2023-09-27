#![allow(dead_code)]
// Using, d3d
use crate::consts;
use crate::math::{clamp, lerp, Vec2, Vec3};
use crate::player::Player;
use crate::windows::draw_pixel;
use crate::world::{World, Sector, Material, TextureMapping, SectorHeight};
use crate::texture::TextureSet;
// Using
use std::rc::Rc;
use num_traits::Zero;
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
struct Surface {
    points: Vec<i32>,
    view: SurfaceView,
    wall_offset: i32,
}

#[derive(Clone)]
struct WallContext {
    wall: [Vec3<i32>; 4],
    depth: [f32; 2],
    uclip: [f32; 2],
    width: f32,
    face: Face,
    distance: i32,
    visiable: bool
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

mod render {
    use lazy_static::lazy_static;

    use crate::consts::{self, H_WIDTH, H_FOV};
    use crate::math::radians;
    use std::f32::consts::PI;
    use libm::atanf;

    lazy_static! {
        pub static ref SCREEN_DIST: f32 = {
            (consts::H_WIDTH as f32) / radians(H_FOV).tan()
        };

        pub static ref X_TO_ANGLE: [f32; (consts::WIDTH+1) as usize] = {
            let mut x_to_angle = [0.0; (consts::WIDTH+1) as usize];
            for x in 0..=consts::WIDTH {
                x_to_angle[x as usize] = atanf((consts::H_WIDTH - x) as f32 / *SCREEN_DIST);
            }
            x_to_angle
        };
    }

    #[inline]
    pub fn inv_fov() -> f32 {
        1.0 / (consts::H_FOV * PI / 180.0).tan()
    }

    #[inline]
    pub fn width_on_fov() -> i32 {
        ((consts::WIDTH as f32) * inv_fov()) as i32
    } 

    pub fn angle_to_x(angle: f32) -> f32 {
        if angle > 0.0 {
            *SCREEN_DIST - angle.tan() * (H_WIDTH as f32)
        } else {
            -angle.tan() * (H_WIDTH as f32) + *SCREEN_DIST
        }
    }
}

impl Surface {

    fn look_and_move_updown(&self, player: &Player) -> (f32, f32) {
        // Looks up and down factor
        let factor = (render::width_on_fov() as f32) / (consts::UPDOWN_FACTOR as f32) - 0.1;
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
        let fy = (render::width_on_fov() as f32) / z * move_updown * tile;
        let psin = player.sin();
        let pcos = player.cos();
        let mut rx = fx * psin - fy * pcos + ((player.position.y as f32)/(yo as f32) * tile); 
        let mut ry: f32 = fx * pcos + fy * psin - ((player.position.x as f32)/(yo as f32) * tile); 
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
                    SurfaceView::Bottom => { 
                        // Example of a bottom view:
                        //       __
                        //      /||\
                        //     /||||\
                        //    ||||||||
                        //    ||||||||
                        //    ||/++\|| 
                        //    |/++++\|
                        //     \++++/
                        //      \++/
                        
                        // Set y2 as the value of the vertical coordinates of the top of the bottom edges,
                        // which are the bottom edges of the front view.
                        y2 = self.points[x as usize];
                        material = materials[2];
                    },
                    SurfaceView::Top => { 
                        // Example of a top view:
                        //       __
                        //      /++\
                        //     /++++\
                        //    |\++++/|
                        //    |||++/||
                        //    |||||||| 
                        //    ||||||||
                        //    \||||||/
                        //     \||||/
                        
                        // Set y1 as the value of the vertical coordinates of the bottom of the top edges,
                        // which are the top edges of the front view.
                        y1 = self.points[x as usize]; 
                        material = materials[1];
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
                    SurfaceView::Bottom => { self.points[x as usize] = y1; }, // save bottom edge of front
                    SurfaceView::Top    => { self.points[x as usize] = y2; }, // save top edge of front
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

impl WallContext {
    
    pub fn new() -> Self {
        WallContext {
            wall: [Vec3::zeros(); 4],
            depth: [0.0; 2],
            uclip: [0.0; 2],
            width: 0.0,
            face: Face::Back,
            distance: 0,
            visiable: false
        }
    }

    pub fn draw(
        &mut self, 
        mut pixels: &mut Pixels, 
        surface: &mut Surface, 
        player: &Player,
        textures: &TextureSet,
        materials: &[&Material; 3]
    ) {
        // Draw only visible surface
        if !self.visiable { return; }
        // y distance of bottom line
        let dyb = self.distance_bottom_line();
        // y distance of top line
        let dyt = self.distance_top_line();
        // x distance
        let dx: i32 = self.large(); if dx == 0 { return; }
        // Hold initial x1 starting position
        let xs: i32 = self.wall[0].x;
        // Let x1 and x2
        let mut x1 = self.wall[0].x;
        let mut x2 = self.wall[1].x;
        // Clip X
        x1 = clamp(x1, 0, consts::WIDTH as i32);
        x2 = clamp(x2, 0, consts::WIDTH as i32);
        // Draw line
        for x in x1..x2 {
            // compute u
            let u = match materials[0] {
                Material::Texture(map) => self.u_texturing_prospective(textures, x - xs, dx, map),
                _ => 0.0
            };
            // From x1 to x, starting from closet point to current bottom
            let mut y1: i32 = dyb * (((x - xs) as f32 + 0.5) as i32) / dx + self.wall[0].y;
            // From x1 to x, starting from closet point to current top
            let mut y2: i32 = dyt * (((x - xs) as f32 + 0.5) as i32) / dx + self.wall[2].y;
            // texture: i32 V
            let (v_coord, v_step)= match materials[0] {
                Material::Texture(map) => self.v_texturing(textures, y1, y2, map),
                _ => (0.0,0.0)
            }; 
            // Clip Y
            y1 = clamp(y1, 0, consts::HEIGHT as i32);
            y2 = clamp(y2, 0, consts::HEIGHT as i32);
            // Draw
            surface.draw(
                &mut pixels,
                &player,
                &self.face, 
                x, u, 
                y1, y2, v_coord, v_step, 
                textures, &materials
            );
        }
    }

    fn project(&mut self, player: &Player, face: &Face, wall2d: &[Vec2<i32>; 2], height: &SectorHeight) -> bool {
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
        // Save wall width
        self.width = wall2d[0].as_vec::<f32>().distance(&wall2d[1].as_vec::<f32>());
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
            self.wall[i].z = ((height.bottom - player.position.z) as f32
                           + ((player.updown * self.wall[i].y) as f32 / consts::UPDOWN_FACTOR))
                           as i32;

            // Second line,  X,Y are the same
            self.wall[i + 2].x = self.wall[i].x;
            self.wall[i + 2].y = self.wall[i].y;
            // Z is to be recompute with new height
            self.wall[i + 2].z = ((height.top - player.position.z) as f32
                               + ((player.updown * self.wall[i].y) as f32 / consts::UPDOWN_FACTOR))
                               as i32;
        }
        // Distance
        self.distance = Vec2::<f32>::zeros().distance(
            &Vec2::new((self.wall[0].x + self.wall[1].x) as f32 / 2.0,
                             (self.wall[0].y + self.wall[1].y) as f32 / 2.0)
        ) as i32;
        // Store w[0], w[1] before clip
        let w_preclip = [
            self.wall[0].clone(), 
            self.wall[1].clone()
        ];
        // Clip wall behind player
        if self.wall[0].y < 1 && self.wall[1].y < 1 {
            self.visiable = false;
            self.distance = 0;
            return self.visiable;
        }
        // Point 1 behind player, clip
        else if self.wall[0].y < 1 {
            self.clip_behind_player(0,1); // bottom line
            self.clip_behind_player(2,3); // top line
        }
        // Point 2 behind player, clip
        else if self.wall[1].y < 1 {
            self.clip_behind_player(1, 0); // bottom line
            self.clip_behind_player(3,2); // top line
        }
        // Save depth, NB. stored in Y coord
        for i in 0..2 {
            self.depth[i] = self.wall[i].y as f32;
            self.uclip[i] = self.wall[i].xy().as_vec::<f32>()
                            .distance(&w_preclip[i].xy().as_vec::<f32>()) / self.width;
        }
        // Screen position
        for i in 0..4 {
            self.wall[i].x = (self.wall[i].x * render::width_on_fov()) / self.wall[i].y + consts::H_WIDTH as i32;
            self.wall[i].y = (self.wall[i].z * render::width_on_fov()) / self.wall[i].y + consts::H_HEIGHT as i32;
        }
        // Draw
        self.visiable = true;
        // Return visiable
        return self.visiable;
    }

    fn clip_behind_player(&mut self, p1: usize, p2: usize) {
        let point2: Vec3<i32> = self.wall[p2].clone();
        let point1: &mut Vec3<i32> = &mut self.wall[p1];
        let da = point1.y as f32;
        let db = point2.y as f32;
        let d = da - db;
        if !d.is_zero() && !d.is_nan() && !d.is_infinite() {
            let s = da / d ;
            if !s.is_zero() && !s.is_nan() && !s.is_infinite() {
                let diff: Vec3<f32> = (point2 - *point1).as_vec::<f32>();
                *point1 = (point1.as_vec::<f32>() + diff * s).as_vec::<i32>();
            }
        }
        if point1.y == 0 { point1.y = 1; }
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

    fn u_texturing_linear(&self, textures: &TextureSet, wx: i32, dx: i32, map: &TextureMapping) -> f32 {
        let a = wx as f32 / (dx as f32);
        let u0 = self.uclip[0];
        let u1 = 1.0 - self.uclip[1];
        let u = lerp(u0,u1,a);
        return u * (map.uv.x as f32) * (textures.set[map.texture].dimensions.x as f32);
    }

    fn u_texturing_prospective(&self, textures: &TextureSet, wx: i32, dx: i32, map: &TextureMapping) -> f32 {
        let a = wx as f32 / (dx as f32);
        let u0 = self.uclip[0];
        let u1 = 1.0 - self.uclip[1];
        let z0 = self.depth[0];
        let z1 = self.depth[1];
        let iz0 = 1.0 / z0;
        let iz1 = 1.0 / z1;
        let utop = lerp(u0 * iz0,u1 * iz1,a);
        let ubottom = lerp(iz0, iz1, a);
        let u = utop / ubottom;
        return u * (map.uv.x as f32) * (textures.set[map.texture].dimensions.x as f32);
    }

    fn v_texturing(&self, textures: &TextureSet, y1: i32, y2:i32, map: &TextureMapping) -> (f32, f32){
        let step = (textures.set[map.texture].dimensions.y as i32 * map.uv.y) as f32 / ((y2-y1) as f32);
        let start: f32 = if y1 < 0 { -step * (y1 as f32) } else { 0.0 };
        return (start,step);
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
        if position.z > sector.height.top {
            self.surface.view = SurfaceView::Top;
            self.surface.wall_offset = sector.height.top;
            self.surface.points.fill(consts::HEIGHT as i32);
            [Face::Front, Face::Back].iter()
        } else if position.z < sector.height.bottom {
            self.surface.view = SurfaceView::Bottom;
            self.surface.wall_offset = sector.height.bottom;
            self.surface.points.fill(0);
            [Face::Front, Face::Back].iter()
        } else {
            self.surface.view = SurfaceView::Mid;
            [Face::Front].iter()
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
        let mut wall_context = WallContext::new();
        // Sort
        self.sectors_context.sort_by(|left, right| right.distance.cmp(&left.distance));    
        // Mut ref to self.sectors_context
        let sectors_context = &mut self.sectors_context;
        // For each sector
        for context in sectors_context {
            // Ref to sector
            let sector = &self.world.sectors[context.index];
            // Let wall count
            let mut count_walls : i32 = 0;
            // Back and front
            for face in context.start(&player.position, &sector) {
                // For each wall
                for wall_id in sector.wall.x..sector.wall.y {
                    // Wall
                    let wall = &self.world.walls[wall_id as usize];
                    // Wall 2D
                    let wall2d = [wall.point1, wall.point2];
                    // Material set
                    let materials = [
                        &wall.material,
                        &sector.material[0],
                        &sector.material[1],
                    ];
                    // From a wall described as two points + height, to 3D world
                    if wall_context.project(&player, &face, &wall2d, &sector.height) {
                        // Draw
                        wall_context.draw(
                            &mut pixels,
                            &mut context.surface, 
                            &player,
                            self.textures.as_ref(),
                            &materials
                        );
                    }
                    // Add distance
                    context.distance += wall_context.distance;
                    count_walls += 1;
                }
            }
            // AVG distance:
            if 0 < count_walls {
                context.distance /= count_walls;
            }
        }
    }
}
