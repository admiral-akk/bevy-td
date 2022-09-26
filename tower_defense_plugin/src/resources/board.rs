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
                Coordinates::new(5, 8),
            ],
        }
    }
    pub fn width(&self) -> u16 {
        self.size.0
    }

    pub fn height(&self) -> u16 {
        self.size.1
    }

    pub fn is_path(&self, coord: &Coordinates) -> bool {
        self.path.contains(coord)
    }
}
