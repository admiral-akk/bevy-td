use bevy::prelude::{Entity, Input, MouseButton, Query, Res, ResMut, With};

use crate::{
    components::{coordinates::Coordinates, cursor::Cursor, selected::Selected, tower::Tower},
    resources::board::{Board},
};

pub fn select_tower(
    cursor: Query<&Cursor>,
    board: Res<Board>,
    mut selected: Query<&mut Selected>,
    mut click: ResMut<Input<MouseButton>>,
    _tower: Query<Entity, With<Tower>>,
) {
    let mut selected = selected.single_mut();
    if selected.0.is_some() {
        return;
    }
    if let Some(cursor) = cursor.single().0 {
        if board.towers.contains_key(&cursor) {
            if click.just_pressed(MouseButton::Left) {
                selected.0 = Some(cursor.clone());
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
            if board.invalid_placement(&cursor_pos) {
                return;
            }
            if click.just_pressed(MouseButton::Left) {
                if let Some(tower) = board.towers.get(&selected_pos) {
                    *towers.get_mut(*tower).unwrap() = cursor_pos.clone();
                }
                selected.0 = None;
                click.clear();
            }
        }
    }
}
