use bevy::prelude::{Entity, Input, MouseButton, Query, Res, ResMut, With};

use crate::{
    components::{
        coordinates::Coordinates, cursor::Cursor, selected::Selected, start::Start, tower::Tower,
    },
    resources::board::Board,
};

pub fn select_tower(
    cursor: Query<&Cursor>,
    board: Res<Board>,
    mut selected: Query<&mut Selected>,
    mut click: ResMut<Input<MouseButton>>,
    tower: Query<Entity, With<Tower>>,
) {
    let mut selected = selected.single_mut();
    if selected.0.is_some() {
        return;
    }
    if let Some(cursor) = cursor.single().0 {
        if let Some(entity) = board.entities.get(&cursor) {
            if let Ok(_) = tower.get(*entity) {
                if click.just_pressed(MouseButton::Left) {
                    selected.0 = Some(cursor.clone());
                    click.clear();
                }
            }
        }
    }
}

pub fn place_tower(
    board: Res<Board>,
    cursor: Query<&Cursor>,
    mut selected: Query<&mut Selected>,
    mut towers: Query<(&mut Coordinates, &mut Start), With<Tower>>,
    mut click: ResMut<Input<MouseButton>>,
) {
    let mut selected = selected.single_mut();
    if let Some(selected_pos) = selected.0 {
        if let Some(cursor_pos) = cursor.single().0 {
            if board.invalid_placement(&cursor_pos) {
                return;
            }
            if click.just_pressed(MouseButton::Left) {
                if let Some(entity) = board.entities.get(&selected_pos) {
                    if let Ok((mut coord, mut start)) = towers.get_mut(*entity) {
                        *coord = cursor_pos.clone();
                        start.0 = cursor_pos.clone();
                    }
                }
                selected.0 = None;
                click.clear();
            }
        }
    }
}
