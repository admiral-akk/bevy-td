use bevy::{prelude::Entity, utils::HashMap};

use crate::components::coordinates::Coordinates;

pub struct Board {
    pub size: (u16, u16),
    pub tiles: HashMap<Coordinates, Entity>,
    pub start: Coordinates,
    pub end: Coordinates,
    pub path: Vec<Coordinates>,
}

impl Board {
    pub fn new() -> Self {
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
            size: (16, 16),
            tiles: HashMap::new(),
            start,
            end,
            path,
        }
    }
    pub fn width(&self) -> u16 {
        self.size.0
    }

    pub fn height(&self) -> u16 {
        self.size.1
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
