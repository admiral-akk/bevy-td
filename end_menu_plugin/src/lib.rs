use assets_plugin::resources::fonts::Fonts;
use bevy::{ecs::schedule::StateData, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use components::{main_menu::MainMenu, new_game::NewGame};
use events::{StartGame, ToMainMenu};
use resources::{main_menu_state::MainMenuState, new_game_state::NewGameState, ui_root::UiRoot};
use systems::{main_menu::main_menu, new_game::new_game};

mod components;
mod events;
mod resources;
mod systems;

pub struct EndMenuPlugin<T> {
    pub active_state: T,
    pub in_game_state: T,
    pub start_menu_state: T,
}

impl<T: StateData> Plugin for EndMenuPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(self.active_state.clone()).with_system(Self::enter));
        app.add_system_set(
            SystemSet::on_update(self.active_state.clone())
                .with_system(main_menu)
                .with_system(new_game)
                .with_system(Self::to_main_menu)
                .with_system(Self::to_new_game),
        );
        app.add_system_set(SystemSet::on_exit(self.active_state.clone()).with_system(Self::exit));
        app.insert_resource(NewGameState(self.in_game_state.clone()));
        app.insert_resource(MainMenuState(self.start_menu_state.clone()));
        app.add_event::<StartGame>();
        app.add_event::<ToMainMenu>();
    }
}

impl<T: StateData> EndMenuPlugin<T> {
    fn to_main_menu(
        mut main_menu_evr: EventReader<ToMainMenu>,
        mut state: ResMut<State<T>>,
        main_menu_state: Res<MainMenuState<T>>,
    ) {
        for _ in main_menu_evr.iter() {
            state.replace(main_menu_state.0.clone()).unwrap();
        }
    }

    fn to_new_game(
        mut start_game_evr: EventReader<StartGame>,
        mut state: ResMut<State<T>>,
        in_game_state: Res<NewGameState<T>>,
    ) {
        for _ in start_game_evr.iter() {
            state.replace(in_game_state.0.clone()).unwrap();
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
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0.5, 0.5, 0.5, 0.5)),
                ..Default::default()
            })
            .insert(Name::new("Start Menu"))
            .with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(40.),
                                height: Val::Percent(20.),
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: UiColor(Color::RED),
                        ..Default::default()
                    })
                    .insert(NewGame)
                    .add_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "New Game".to_string(),
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
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(40.),
                                height: Val::Percent(20.),
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: UiColor(Color::RED),
                        ..Default::default()
                    })
                    .insert(MainMenu)
                    .add_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "Start Menu".to_string(),
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

    fn exit(mut commands: Commands, ui_root: Res<UiRoot>) {
        commands.entity(ui_root.0).despawn_recursive();
    }
}
