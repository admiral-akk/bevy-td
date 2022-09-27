use bevy::{prelude::Handle, sprite::TextureAtlas};

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

pub struct PathSprites {
    pub path_atlas_handle: Option<Handle<TextureAtlas>>,
}

impl PathSprites {
    pub fn get_spawn_index() -> usize {
        11 + 8 * 23
    }
    pub fn get_end_index() -> usize {
        13 + 8 * 23
    }

    pub fn get_grass_index(coord: &Coordinates) -> usize {
        (coord.x * 3 / 2 + coord.y * 5 / 2) as usize % 2
    }

    pub fn get_path_index(coord: &Coordinates, board: &Board) -> usize {
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

        PATH_INDEX[index]
    }
}
