use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::{BuildChildren, Commands, Entity, EventReader, MouseButton, Query, Res, With},
};

use crate::{
    components::{
        coordinates::Coordinates,
        cursor::{Cursor},
        selected::Selected,
        tower::Tower,
    },
    resources::board::Board,
};

fn is_left_click(input: &MouseButtonInput) -> bool {
    input.button == MouseButton::Left && input.state == ButtonState::Pressed
}

pub fn select_tower(
    mut commands: Commands,
    board: Res<Board>,
    cursor: Query<&Cursor>,
    mut click_evr: EventReader<MouseButtonInput>,
) {
    if let Some(coord) = cursor.single().0 {
        for event in click_evr.iter() {
            if is_left_click(&event) {
                let selected = commands.spawn().insert(Selected(coord.clone())).id();
                commands.entity(board.board.unwrap()).add_child(selected);
            }
        }
    }
}

pub fn place_tower(
    mut commands: Commands,
    board: Res<Board>,
    cursor: Query<&Cursor>,
    selected: Query<(Entity, &Selected)>,
    mut towers: Query<&mut Coordinates, With<Tower>>,
    mut click_evr: EventReader<MouseButtonInput>,
) {
    if selected.is_empty() {
        return;
    }
    if let Some(cursor_pos) = cursor.single().0 {
        let (selected, coord) = selected.single();
        for event in click_evr.iter() {
            if is_left_click(&event) {
                if let Some(tower) = board.towers.get(&coord.0) {
                    *towers.get_mut(*tower).unwrap() = cursor_pos.clone();
                }
                commands.entity(selected).despawn();
            }
        }
    }
}
