use bevy::prelude::{Entity, Transform, Vec2, Vec3};

use crate::components::coordinates::Coordinates;

use super::bimap::BiMap;

pub struct Board {
    pub size: (u16, u16),
    pub tiles: BiMap<Coordinates, Entity>,
    pub towers: BiMap<Coordinates, Entity>,
    pub monsters: BiMap<Coordinates, Entity>,
    pub start: Coordinates,
    pub end: Coordinates,
    pub path: Vec<Coordinates>,
    pub tile_size: f32,
    pub board: Option<Entity>,
}

pub enum TileType {
    None,
    Grass,
    Dirt,
    Road,
    Start,
    Finish,
    Bench,
    Trainer,
    Trainee,
    Result,
    Arrow,
}

impl Board {
    pub fn new(size: (u16, u16), tile_size: f32) -> Self {
        let start = Coordinates::new(2, 8);
        let end = Coordinates::new(10, 12);
        let mut path = Vec::new();
        let mut path_coord = start.clone();
        while path_coord.x != end.x {
            path.push(path_coord);
            if path_coord.x > end.x {
                path_coord.x -= 1;
            } else {
                path_coord.x += 1;
            }
        }
        while path_coord.y != end.y {
            path.push(path_coord);
            if path_coord.y > end.y {
                path_coord.y -= 1;
            } else {
                path_coord.y += 1;
            }
        }
        path.push(path_coord);
        Board {
            size,
            tiles: BiMap::new(),
            towers: BiMap::new(),
            monsters: BiMap::new(),
            start,
            end,
            path,
            tile_size,
            board: None,
        }
    }

    pub fn invalid_placement(&self, coordinates: &Coordinates) -> bool {
        match self.tile_type(coordinates) {
            TileType::None
            | TileType::Arrow
            | TileType::Finish
            | TileType::Road
            | TileType::Result
            | TileType::Start
            | TileType::Dirt => {
                return true;
            }
            _ => {}
        }
        return false;
    }

    pub fn tile_type(&self, coordinates: &Coordinates) -> TileType {
        if coordinates.y == 0 {
            return TileType::Bench;
        } else if coordinates.y == 1 {
            return TileType::None;
        } else if coordinates.x < 16 {
            if coordinates.y < 10 {
                return TileType::Grass;
            } else {
                return TileType::Dirt;
            }
        } else if coordinates.x == 16 {
            return TileType::None;
        } else {
            let trainer_index = self.size.1 - coordinates.y - 1;
            if trainer_index > self.size.1 - 2 {
                return TileType::None;
            }
            match trainer_index % 3 {
                0 => match coordinates.x - 17 {
                    1 => {
                        return TileType::Trainer;
                    }
                    _ => {
                        return TileType::None;
                    }
                },
                1 => match coordinates.x - 17 {
                    0 => {
                        return TileType::Trainee;
                    }
                    1 => {
                        return TileType::Arrow;
                    }
                    _ => {
                        return TileType::Result;
                    }
                },
                _ => {
                    return TileType::None;
                }
            }
        }
    }

    pub fn empty(&self, coord: &Coordinates) -> bool {
        !self.monsters.contains_key(coord) && !self.towers.contains_key(coord)
    }

    pub fn width(&self) -> u16 {
        self.size.0
    }

    pub fn height(&self) -> u16 {
        self.size.1
    }

    pub fn board_offset(&self) -> Vec3 {
        let offset = -0.5 * self.board_size();
        Vec3::new(offset.x, offset.y, 0.)
    }

    pub fn next(&self, coord: &Coordinates) -> Coordinates {
        let index = self.path.iter().position(|c| c.eq(&coord));
        if let Some(index) = index {
            self.path[index + 1]
        } else {
            self.start
        }
    }

    pub fn transform(&self, coord: &Coordinates, z: f32) -> Transform {
        Transform::from_xyz(
            (coord.x as f32 + 0.5) * self.tile_size,
            (coord.y as f32 + 0.5) * self.tile_size,
            z,
        )
    }

    pub fn board_size(&self) -> Vec2 {
        Vec2::new(
            self.tile_size * self.width() as f32,
            self.tile_size * self.height() as f32,
        )
    }

    pub fn is_start(&self, coord: &Coordinates) -> bool {
        self.start.eq(coord)
    }
    pub fn is_end(&self, coord: &Coordinates) -> bool {
        self.end.eq(coord)
    }

    pub fn is_path(&self, coord: &Coordinates) -> bool {
        self.path.contains(coord)
    }

    pub fn neighbouring_monsters(&self, coord: &Coordinates) -> Vec<Coordinates> {
        return coord
            .orthogonal_neighbours()
            .into_iter()
            .filter(|coord| self.monsters.contains_key(coord))
            .collect();
    }

    pub fn neighbouring_paths(&self, coord: &Coordinates) -> Vec<Coordinates> {
        return coord
            .orthogonal_neighbours()
            .into_iter()
            .filter(|coord| self.path.contains(coord))
            .collect();
    }
}
