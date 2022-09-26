use bevy::{
    prelude::{AssetServer, Assets, Handle, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use crate::components::coordinates::Coordinates;

use super::board::Board;

pub struct PathSprites {
    pub path_atlas_handle: Option<Handle<TextureAtlas>>,
}

impl PathSprites {
    pub fn get_path_index(&self, coord: &Coordinates, board: &Board) -> usize {
        return 11;
    }
}
