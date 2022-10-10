use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Default, Clone, Component)]
pub struct HealthBar {
    pub width: f32,
}
impl HealthBar {
    pub fn new(width: f32) -> Self {
        Self { width }
    }
}
