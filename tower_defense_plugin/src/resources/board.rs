use bevy::{
    prelude::{Entity, Vec2, Vec3},
    utils::HashMap,
};

use crate::components::coordinates::Coordinates;

pub struct Board {
    pub size: (u16, u16),
    pub tiles: HashMap<Coordinates, Entity>,
    pub start: Coordinates,
    pub end: Coordinates,
    pub path: Vec<Coordinates>,
    pub tile_size: f32,
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
            tiles: HashMap::new(),
            start,
            end,
            path,
            tile_size,
        }
    }
    pub fn width(&self) -> u16 {
        self.size.0
    }

    pub fn height(&self) -> u16 {
        self.size.1
    }

    pub fn board_offset(&self) -> Vec3 {
        let offset = -self.board_size() / 2.;
        Vec3::new(offset.x, offset.y, 0.)
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

    pub fn neighbouring_paths(&self, coord: &Coordinates) -> Vec<Coordinates> {
        return coord
            .orthogonal_neighbours()
            .into_iter()
            .filter(|coord| self.path.contains(coord))
            .collect();
    }
}
