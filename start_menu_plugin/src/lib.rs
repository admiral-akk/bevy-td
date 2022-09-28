use bevy::{ecs::schedule::StateData, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
pub struct StartMenuPlugin<T> {
    pub active_state: T,
    pub in_game_state: T,
}

impl<T: StateData> Plugin for StartMenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(self.active_state.clone()).with_system(Self::enter));
        app.add_system_set(
            SystemSet::on_update(self.active_state.clone()).with_system(Self::update),
        );
        app.add_system_set(SystemSet::on_exit(self.active_state.clone()).with_system(Self::exit));
    }
}

impl<T> StartMenuPlugin<T> {
    pub fn enter(mut commands: Commands) {
        commands
            .spawn()
            .insert(Name::new("Start Menu"))
            .insert_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    ..Default::default()
                },
                color: UiColor(Color::WHITE),
                ..Default::default()
            });
    }
    pub fn update() {}
    pub fn exit() {}
}
