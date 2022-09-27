use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::{EventReader, EventWriter, MouseButton, Res, ResMut, Vec2},
    window::{CursorMoved, Windows},
};

use crate::{
    components::coordinates::Coordinates,
    events::{EnterBuildTarget, HideBuildTarget, TryBuild},
    resources::{board::Board, build_tracker::BuildTracker},
};

pub fn mouse_move_on_board(
    windows: Res<Windows>,
    board: Res<Board>,
    mut build_tracker: ResMut<BuildTracker>,
    mut cursor_evr: EventReader<CursorMoved>,
    mut set_target_ewr: EventWriter<EnterBuildTarget>,
    mut clear_target_ewr: EventWriter<HideBuildTarget>,
) {
    let window = windows.get_primary().unwrap();
    for event in cursor_evr.iter() {
        let pos = event.position;
        let pos = pos - Vec2::new(window.width(), window.height()) / 2. + board.board_size() / 2.;
        if pos.x < 0. || pos.y < 0. {
            build_tracker.clear_target(&mut clear_target_ewr);
            continue;
        }
        let coord = Coordinates::new(
            (pos.x / board.tile_size) as u16,
            (pos.y / board.tile_size) as u16,
        );
        if coord.x >= board.size.0 || coord.y >= board.size.1 {
            build_tracker.clear_target(&mut clear_target_ewr);
            continue;
        }
        if board.towers.contains_key(&coord) {
            build_tracker.clear_target(&mut clear_target_ewr);
            continue;
        }
        if build_tracker.target.is_some() && build_tracker.target.unwrap().eq(&coord) {
            continue;
        }
        build_tracker.set_target(coord, &mut set_target_ewr);
    }
}

pub fn mouse_click_on_board(
    build_tracker: Res<BuildTracker>,
    mut click_evr: EventReader<MouseButtonInput>,
    mut try_build_ewr: EventWriter<TryBuild>,
) {
    for event in click_evr.iter() {
        if build_tracker.target.is_none() {
            continue;
        }
        match event.state {
            ButtonState::Pressed => match event.button {
                MouseButton::Left => {
                    try_build_ewr.send(TryBuild);
                }
                _ => (),
            },
            _ => {}
        }
    }
}
