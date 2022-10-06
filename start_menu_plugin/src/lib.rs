mod components;
mod events;
mod resources;
mod systems;

use assets_plugin::resources::fonts::Fonts;
use bevy::{ecs::schedule::StateData, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use components::start::Start;
use events::StartGame;
use resources::{in_game_state::InGameState, ui_root::UiRoot};
use systems::start::start;

pub struct StartMenuPlugin<T> {
    pub active_state: T,
    pub in_game_state: T,
}

impl<T: StateData> Plugin for StartMenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(self.active_state.clone()).with_system(Self::enter));
        app.insert_resource(InGameState(self.in_game_state.clone()));
        app.add_system_set(
            SystemSet::on_update(self.active_state.clone())
                .with_system(start)
                .with_system(Self::start_game),
        );
        app.add_system_set(SystemSet::on_exit(self.active_state.clone()).with_system(Self::exit));
        app.add_event::<StartGame>();
    }
}

impl<T: StateData> StartMenuPlugin<T> {
    fn start_game(
        mut start_game_evr: EventReader<StartGame>,
        mut state: ResMut<State<T>>,
        in_game_state: Res<InGameState<T>>,
    ) {
        for _ in start_game_evr.iter() {
            state.set(in_game_state.0.clone()).unwrap();
        }
    }

    fn enter(mut commands: Commands, fonts: Res<Fonts>) {
        let root_node = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: UiColor(Color::WHITE),
                ..Default::default()
            })
            .insert(Name::new("Start Menu"))
            .with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size {
                                width: Val::Px(400.),
                                height: Val::Px(100.),
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: UiColor(Color::RED),
                        ..Default::default()
                    })
                    .insert(Start)
                    .add_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "Begin".to_string(),
                                    style: TextStyle {
                                        font: fonts.get_handle(),
                                        font_size: 128.,
                                        ..Default::default()
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            })
            .id();
        commands.insert_resource(UiRoot(root_node));
    }
    fn exit(mut commands: Commands, root_node: Res<UiRoot>) {
        commands.entity(root_node.0).despawn_recursive();
    }
}
