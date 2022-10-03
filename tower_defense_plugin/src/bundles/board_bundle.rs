use bevy::{
    prelude::{Bundle, Name, Transform, Vec3, VisibilityBundle},
    transform::TransformBundle,
};

use crate::components::board::Board;

#[derive(Bundle)]
pub struct BoardBundle {
    name: Name,
    board: Board,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}

impl BoardBundle {
    pub fn new(board_offset: Vec3) -> Self {
        BoardBundle {
            name: Name::new("Board"),
            board: Board,
            transform: TransformBundle::from_transform(Transform::from_translation(board_offset)),
            visibility: VisibilityBundle::default(),
        }
    }
}
