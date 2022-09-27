use bevy::{
    prelude::{Handle, Transform, Vec2},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};

use crate::components::coordinates::Coordinates;

use super::board::Board;

// binary encoding, NWSE, 1 if has path, 0 otherwise. 0000 means ignore

const PATH_INDEX: [usize; 16] = [
    0,
    13 + 2 * 23, // N
    12 + 2 * 23, // W
    10 + 2 * 23, // NW
    14 + 2 * 23, // S
    10,          // NS
    10 + 23,     // WS
    14 + 23,     // NWS
    12 + 23,     // E
    11 + 2 * 23, // NE
    11,          // WE
    14,          // NWE
    11 + 23,     // SE
    13 + 23,     // NSE
    13,          // WSE
    12,          //NWSE
];

pub struct GameSprites {
    game_sprite_handle: Option<Handle<TextureAtlas>>,
}

impl GameSprites {
    pub fn init() -> Self {
        GameSprites {
            game_sprite_handle: None,
        }
    }

    pub fn update_handle(&mut self, handle: Handle<TextureAtlas>) {
        self.game_sprite_handle = Some(handle);
    }

    fn get_handle(&self) -> Handle<TextureAtlas> {
        self.game_sprite_handle.as_ref().unwrap().clone()
    }

    pub fn spawn(&self, tile_size: f32) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 11 + 8 * 23,
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            texture_atlas: self.get_handle(),
            transform: Transform::from_xyz(0., 0.5 * tile_size, 2.),
            ..Default::default()
        }
    }
    pub fn end(&self, tile_size: f32) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 13 + 8 * 23,
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            texture_atlas: self.get_handle(),
            transform: Transform::from_xyz(0., 0., 2.),
            ..Default::default()
        }
    }

    pub fn peasant(&self, tile_size: f32) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 17 + 6 * 23,
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            texture_atlas: self.get_handle(),
            transform: Transform::from_xyz(0., 0., 3.),
            ..Default::default()
        }
    }
    pub fn monster(&self, tile_size: f32) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 18 + 3 * 23,
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            texture_atlas: self.get_handle(),
            transform: Transform::from_xyz(0., 0., 4.),
            ..Default::default()
        }
    }

    pub fn grass(&self, coord: &Coordinates, tile_size: f32) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: (coord.x * 3 / 2 + coord.y * 5 / 2) as usize % 2,
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            texture_atlas: self.get_handle(),
            transform: Transform::from_xyz(
                coord.x as f32 * tile_size + tile_size / 2.,
                coord.y as f32 * tile_size + tile_size / 2.,
                1.,
            ),
            ..Default::default()
        }
    }

    pub fn path(&self, coord: &Coordinates, board: &Board, tile_size: f32) -> SpriteSheetBundle {
        let mut index = 0;
        let coord = coord.clone();
        if board.is_path(&(coord + Coordinates::new(0, 1))) {
            index = index + 1;
        }
        if board.is_path(&(coord + Coordinates::new(1, 0))) {
            index = index + 2;
        }

        if board.is_path(&(coord - Coordinates::new(0, 1))) {
            index = index + 4;
        }

        if board.is_path(&(coord - Coordinates::new(1, 0))) {
            index = index + 8;
        }

        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: PATH_INDEX[index],
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                ..Default::default()
            },
            texture_atlas: self.get_handle(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }
}
