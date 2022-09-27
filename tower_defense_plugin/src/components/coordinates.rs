use bevy::prelude::Component;
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Sub},
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Self {
        Coordinates { x, y }
    }

    pub fn orthogonal_neighbours(&self) -> Vec<Coordinates> {
        return vec![
            Coordinates::new(self.x, self.y - 1),
            Coordinates::new(self.x, self.y + 1),
            Coordinates::new(self.x - 1, self.y),
            Coordinates::new(self.x + 1, self.y),
        ];
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Coordinates::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Coordinates::new(self.x - rhs.x, self.y - rhs.y)
    }
}