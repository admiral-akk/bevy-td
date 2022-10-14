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

    pub fn orthogonal_neighbours(&self, max_dist: i32) -> Vec<Coordinates> {
        let mut coords = Vec::new();
        for y in -max_dist..=max_dist {
            let remain = max_dist - y.abs();
            for x in -remain..=remain {
                coords.push(Coordinates::new(
                    (self.x as i32 + x) as u16,
                    (self.y as i32 + y) as u16,
                ));
            }
        }
        coords
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
