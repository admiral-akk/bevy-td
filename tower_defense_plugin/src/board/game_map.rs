use crate::components::coordinates::Coordinates;

#[derive(Debug, Clone)]
pub struct GameMap {
    size: (u16, u16),
    spawn: Coordinates,
    end: Coordinates,
}

impl GameMap {
    pub fn empty(width: u16, height: u16, spawn: Coordinates, end: Coordinates) -> Self {
        GameMap {
            size: (width, height),
            spawn,
            end,
        }
    }

    pub fn width(&self) -> u16 {
        self.size.0
    }

    pub fn height(&self) -> u16 {
        self.size.1
    }
}
