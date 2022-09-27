use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    prelude::{EventReader, MouseButton, Res, Vec2},
    window::Windows,
};

use crate::{components::coordinates::Coordinates, resources::board::Board};

pub fn handle_mouse_click(
    windows: Res<Windows>,
    board: Res<Board>,
    mut click_evr: EventReader<MouseButtonInput>,
) {
    let window = windows.get_primary().unwrap();
    for event in click_evr.iter() {
        if let ButtonState::Pressed = event.state {
            let cursor_pos = window.cursor_position();
            if let Some(pos) = cursor_pos {
                let pos =
                    pos - Vec2::new(window.width(), window.height()) / 2. + board.board_size() / 2.;
                if pos.x < 0. || pos.y < 0. {
                    break;
                }
                let coord = Coordinates::new(
                    (pos.x / board.tile_size) as u16,
                    (pos.y / board.tile_size) as u16,
                );
                if coord.x > board.size.0 || coord.y > board.size.1 {
                    break;
                }
                match event.button {
                    MouseButton::Left => {
                        log::info!("Clicked on {}, {}", pos, coord);
                    }
                    _ => (),
                }
            }
        }
    }
}
