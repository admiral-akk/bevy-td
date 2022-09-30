use bevy::{
    prelude::{EventReader, Query, Res, Vec2},
    window::{CursorMoved, Windows},
};

use crate::{
    components::{coordinates::Coordinates, cursor::Cursor},
    resources::board::Board,
};

pub fn cursor_move(
    windows: Res<Windows>,
    board: Res<Board>,
    mut cursor: Query<&mut Cursor>,
    mut cursor_evr: EventReader<CursorMoved>,
) {
    let window = windows.get_primary().unwrap();
    for event in cursor_evr.iter() {
        let mut cursor = cursor.single_mut();
        let pos = event.position;
        let pos = pos - Vec2::new(window.width(), window.height()) / 2. + board.board_size() / 2.;
        if pos.x < 0. || pos.y < 0. {
            cursor.0 = None;
            continue;
        }
        let coord = Coordinates::new(
            (pos.x / board.tile_size) as u16,
            (pos.y / board.tile_size) as u16,
        );
        if coord.x >= board.size.0 || coord.y >= board.size.1 {
            cursor.0 = None;
            continue;
        }
        cursor.0 = Some(coord);
    }
}
