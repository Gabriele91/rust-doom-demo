#![allow(dead_code)]
// Using, d3d
use crate::math::{Vec2, Vec3};
use crate::player::Player;
use crate::world::{Sector, Wall, World};
// Usings
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Map {
    pub player: Rc<RefCell<Player>>,
    pub world: Rc<World>,
}

impl Map {
    pub fn from(path: &str) -> Option<Map> {
        if let Ok(file) = File::open(path) {
            // Reader buffer
            let reader = BufReader::new(file);
            // Get all lines
            let mut lines = reader.lines();
            // Sectors
            let mut sectors: Vec<Sector> = Vec::new();
            // Read number of sectors
            let number_of_sectors: i32 = match lines.next() {
                Some(Ok(line)) => line.trim().parse().unwrap_or(0),
                _ => 0,
            };
            for _ in 0..number_of_sectors {
                let numbers: Vec<i32> = match lines.next() {
                    Some(Ok(line)) => line
                        .split_whitespace()
                        .map(|s| s.parse().expect("Failed to parse number"))
                        .collect(),
                    _ => vec![],
                };
                if numbers.len() < 4 {
                    return None;
                }
                sectors.push(Sector::new(
                    &Vec2::new(numbers[0], numbers[1]),
                    &Vec2::new(numbers[2], numbers[3]),
                ));
            }
            // Walls
            let mut walls: Vec<Wall> = Vec::new();
            // Number of walls
            let number_of_walls: i32 = match lines.next() {
                Some(Ok(line)) => line.trim().parse().unwrap_or(0),
                _ => 0,
            };
            for _ in 0..number_of_walls {
                let numbers: Vec<i32> = match lines.next() {
                    Some(Ok(line)) => line
                        .split_whitespace()
                        .map(|s| s.parse().expect("Failed to parse number"))
                        .collect(),
                    _ => vec![],
                };
                if numbers.len() < 4 {
                    return None;
                }
                walls.push(Wall::new(
                    &Vec2::new(numbers[0], numbers[1]),
                    &Vec2::new(numbers[2], numbers[3]),
                ));
            }
            // Build world
            let world = Rc::new(World {
                walls: walls,
                sectors: sectors,
            });

            // Read void line
            let void_line = match lines.next() {
                Some(Ok(line)) => String::from(line.trim()),
                _ => return None,
            };
            if !void_line.is_empty() {
                return None;
            }

            // Read player
            let player_numbers: Vec<i32> = match lines.next() {
                Some(Ok(line)) => line
                    .split_whitespace()
                    .map(|s| s.parse().expect("Failed to parse number"))
                    .collect(),
                _ => vec![],
            };
            if player_numbers.len() < 5 {
                return None;
            }
            let player = Rc::new(RefCell::new(Player {
                position: Vec3::new(player_numbers[0], player_numbers[1], player_numbers[2]),
                angle: player_numbers[3],
                updown: player_numbers[4],
            }));

            // Return the
            return Some(Map {
                player: player,
                world: world,
            });
        } else {
            return None;
        }
    }
}
