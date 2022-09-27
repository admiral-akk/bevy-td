use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    prelude::{EventReader, MouseButton, Res, ResMut, Vec2},
    window::{CursorMoved, Windows},
};

use crate::{
    components::coordinates::Coordinates,
    resources::{
        board::Board,
        hover_coordinate::{self, HoverCoordinate},
    },
};

pub fn handle_mouse(
    windows: Res<Windows>,
    board: Res<Board>,
    mut hover_coordinate: ResMut<HoverCoordinate>,
    mut click_evr: EventReader<MouseButtonInput>,
    mut cursor_evr: EventReader<CursorMoved>,
) {
    let window = windows.get_primary().unwrap();
    for event in cursor_evr.iter() {
        let pos = event.position;
        let pos = pos - Vec2::new(window.width(), window.height()) / 2. + board.board_size() / 2.;
        if pos.x < 0. || pos.y < 0. {
            hover_coordinate.0 = None;
            break;
        }
        let coord = Coordinates::new(
            (pos.x / board.tile_size) as u16,
            (pos.y / board.tile_size) as u16,
        );
        if coord.x >= board.size.0 || coord.y >= board.size.1 {
            hover_coordinate.0 = None;
            break;
        }
        if hover_coordinate.0.is_none() || !hover_coordinate.0.unwrap().eq(&coord) {
            hover_coordinate.0 = Some(coord);
        }
    }
    if hover_coordinate.0.is_none() {
        return;
    }
    for event in click_evr.iter() {
        if let ButtonState::Pressed = event.state {
            match event.button {
                MouseButton::Left => {
                    log::info!("Clicked on {}", hover_coordinate.0.unwrap());
                }
                _ => (),
            }
        }
    }
}
