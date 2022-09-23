use crate::events::TileMarkEvent;
use crate::events::TileTriggerEvent;
use crate::Board;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::log;
use bevy::prelude::*;
pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut tile_mark_ewr: EventWriter<TileMarkEvent>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            tile_mark_ewr.send(TileMarkEvent(coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
