use bevy::{
    prelude::{Bundle, Name, Transform, VisibilityBundle},
    transform::TransformBundle,
};

use crate::components::{coordinates::Coordinates, tile::Tile};

#[derive(Bundle)]
pub struct TileBundle {
    name: Name,
    tile: Tile,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}

impl TileBundle {
    pub fn new(coordinates: Coordinates, tile_offset: Transform) -> Self {
        TileBundle {
            name: Name::new(format!("Tile {}, {}", coordinates.x, coordinates.y)),
            tile: Tile,
            transform: TransformBundle::from_transform(tile_offset),
            visibility: VisibilityBundle::default(),
        }
    }
}
