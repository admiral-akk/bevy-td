use bevy::prelude::{Input, MouseButton, Query, Res, ResMut, With};

use crate::{
    components::{coordinates::Coordinates, cursor::Cursor, selected::Selected, tower::Tower},
    resources::board::{Board, TileType},
};

pub fn select_tower(
    cursor: Query<&Cursor>,
    board: Res<Board>,
    mut selected: Query<&mut Selected>,
    mut click: ResMut<Input<MouseButton>>,
) {
    let mut selected = selected.single_mut();
    if selected.0.is_some() {
        return;
    }
    if let Some(cursor) = cursor.single().0 {
        if board.towers.contains_key(&cursor) {
            if click.just_pressed(MouseButton::Left) {
                selected.0 = Some(cursor.clone());
                bevy::log::error!("Selected: {:?}", cursor);
                click.clear();
            }
        }
    }
}

pub fn place_tower(
    board: Res<Board>,
    cursor: Query<&Cursor>,
    mut selected: Query<&mut Selected>,
    mut towers: Query<&mut Coordinates, With<Tower>>,
    mut click: ResMut<Input<MouseButton>>,
) {
    let mut selected = selected.single_mut();
    if let Some(selected_pos) = selected.0 {
        if let Some(cursor_pos) = cursor.single().0 {
            match board.tile_type(&cursor_pos) {
                TileType::None
                | TileType::Arrow
                | TileType::Finish
                | TileType::Road
                | TileType::Result
                | TileType::Start => {
                    return;
                }
                _ => {}
            }
            if click.just_pressed(MouseButton::Left) {
                if let Some(tower) = board.towers.get(&selected_pos) {
                    *towers.get_mut(*tower).unwrap() = cursor_pos.clone();
                    bevy::log::error!("Moved tower to: {:?}", cursor_pos);
                }
                selected.0 = None;
                click.clear();
            }
        }
    }
}
