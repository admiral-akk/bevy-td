use bevy::{
    ecs::schedule::StateData,
    prelude::{Plugin, SystemSet},
};

pub struct EndMenuPlugin<T> {
    pub active_state: T,
    pub in_game_state: T,
    pub start_menu_state: T,
}

impl<T: StateData> Plugin for EndMenuPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(self.active_state.clone()).with_system(Self::enter));
        app.add_system_set(SystemSet::on_update(self.active_state.clone()));
        app.add_system_set(SystemSet::on_exit(self.active_state.clone()).with_system(Self::exit));
    }
}

impl<T: StateData> EndMenuPlugin<T> {
    fn enter() {}
    fn exit() {}
}
