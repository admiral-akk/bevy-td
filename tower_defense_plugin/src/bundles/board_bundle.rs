use bevy::{
    prelude::{Bundle, Name, Transform, Vec3, VisibilityBundle},
    transform::TransformBundle,
};

#[derive(Bundle)]
pub struct BoardBundle {
    name: Name,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    visibility: VisibilityBundle,
}

impl BoardBundle {
    pub fn new(board_offset: Vec3) -> Self {
        BoardBundle {
            name: Name::new("Board"),
            transform: TransformBundle::from_transform(Transform::from_translation(board_offset)),
            visibility: VisibilityBundle::default(),
        }
    }
}
