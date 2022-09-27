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
        Board {
            size: (16, 16),
            tiles: HashMap::new(),
            start: Coordinates::new(2, 8),
            end: Coordinates::new(5, 8),
            path: vec![
                Coordinates::new(2, 8),
                Coordinates::new(3, 8),
                Coordinates::new(4, 8),
                Coordinates::new(4, 9),
                Coordinates::new(4, 7),
                Coordinates::new(5, 8),
                Coordinates::new(6, 8),
                Coordinates::new(7, 8),
                Coordinates::new(8, 8),
                Coordinates::new(8, 9),
                Coordinates::new(9, 9),
                Coordinates::new(10, 9),
                Coordinates::new(10, 8),
                Coordinates::new(10, 7),
                Coordinates::new(10, 6),
                Coordinates::new(10, 5),
                Coordinates::new(9, 5),
                Coordinates::new(8, 5),
                Coordinates::new(8, 6),
            ],
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
