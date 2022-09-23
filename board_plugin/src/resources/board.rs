use crate::resources::bounds::Bounds2;
use crate::{Coordinates, TileMap};
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub flagged_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
}

impl Board {
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;
        }
        // World space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    /// Retrieves a covered tile entity
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        if self.flagged_tiles.contains_key(coords) {
            None
        } else {
            self.covered_tiles.get(coords)
        }
    }
    fn unmark_tile(&mut self, coords: &Coordinates) -> Option<Coordinates> {
        if self.covered_tiles.contains_key(coords) {
            self.covered_tiles.remove(coords);
            Some(coords.clone())
        } else {
            log::error!("Failed to unmark tile at {}", coords);
            None
        }
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        if self.flagged_tiles.contains_key(coords) {
            self.unmark_tile(coords)?;
        }
        self.covered_tiles.remove(coords)
    }

    pub fn try_toggle_flag(&mut self, coords: &Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coords)?;
        let mark = if self.flagged_tiles.contains_key(coords) {
            self.unmark_tile(coords)?;
            false
        } else {
            self.flagged_tiles.insert(*coords, entity);
            true
        };
        Some((entity, mark))
    }

    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }

    /// We retrieve the adjacent covered tile entities of `coord`
    pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}
