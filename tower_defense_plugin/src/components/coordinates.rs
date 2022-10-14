use bevy::prelude::Component;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    ops::{Add, Sub},
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Coordinates {
    pub fn new(x: i16, y: i16) -> Self {
        Coordinates { x, y }
    }

    pub fn distance_field(
        targets: &[Coordinates],
        obstacles: &[Coordinates],
    ) -> HashMap<Coordinates, u32> {
        let mut q1: HashSet<Coordinates> = targets.iter().map(|coord| *coord).collect();
        let mut q2 = HashSet::new();
        let mut distance = HashMap::new();
        let mut dist = 0;

        while !q1.is_empty() || !q2.is_empty() {
            for next in q1.iter() {
                distance.insert(*next, dist);
                for neighbour in next.orthogonal_neighbours(1) {
                    if distance.contains_key(&neighbour) {
                        continue;
                    }
                    if obstacles.contains(&neighbour) {
                        continue;
                    }
                    q2.insert(neighbour);
                }
            }
            q1 = q2;
            dist += 1;
            q2 = HashSet::new();
        }
        distance
    }

    pub fn orthogonal_neighbours(&self, max_dist: i16) -> Vec<Coordinates> {
        let mut coords = Vec::new();
        for y in -max_dist..=max_dist {
            let remain = max_dist - y.abs();
            for x in -remain..=remain {
                let coord = Coordinates::new(self.x + x, self.y + y);
                if coord.x > 16 {
                    continue;
                }
                if coord.y > 18 {
                    continue;
                }
                if coord.x < 0 {
                    continue;
                }
                if coord.y < 2 {
                    continue;
                }
                coords.push(coord);
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
