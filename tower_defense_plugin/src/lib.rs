mod board;

use bevy::{
    prelude::{App, Commands, Plugin, Res},
    window::WindowDescriptor,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
pub struct TowerDefensePlugin {}

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
    }
}

impl TowerDefensePlugin {
    pub fn create_board(mut commands: Commands, window: Res<WindowDescriptor>) {}
}
