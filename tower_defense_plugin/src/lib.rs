mod bundles;
mod components;
mod entities;
mod events;
mod plugins;
pub mod resources;
mod stages;
mod systems;

use std::collections::VecDeque;

use assets_plugin::resources::{
    fonts::Fonts,
    heroes::{HeroSprites, HeroType},
};
use bevy::{
    ecs::schedule::{ShouldRun, StateData},
    prelude::*,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use bundles::{board_bundle::BoardBundle, tile_bundle::TileBundle};
use components::{
    coordinates::Coordinates, cursor::Cursor, go::Go, selected::Selected, turn_order::TurnOrder,
};

use entities::heroes::add_hero;
use events::{
    ActiveAction, ActiveUnit, AttackEvent, EnterBuildTarget, GameOver, HideBuildTarget, Removed,
    StartWave, TryBuild,
};
use plugins::{events::Reward, reward_plugin::RewardPlugin};
use resources::{
    board::{Board, TileType},
    game_sprites::GameSprites,
    game_step_timer::GameStepTimer,
};
use stages::action::ActionStage;
use systems::{
    action::add_action,
    coordinates::{added, removed, return_to_start, updated},
    cursor::cursor_move,
    go::{enable, go, grey_out},
    health::add_health_bar,
    selected::{place_tower, select_tower},
    spawn_wave::monster_spawn,
    turn_order::{add_turn, remove_turn, reset_turn},
};

pub struct TowerDefensePlugin<T> {
    pub active_state: T,
    pub end_menu_state: T,
}

pub struct EndMenuState<T>(pub T);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    None,
    Building,
    Fighting,
    Reward,
}

fn is_fighting(state: Res<State<GameState>>) -> ShouldRun {
    match state.current() {
        GameState::Fighting => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

fn fighting_system_set() -> SystemSet {
    SystemSet::new().with_run_criteria(is_fighting)
}

fn in_game_system_set() -> SystemSet {
    SystemSet::new().with_run_criteria(|state: Res<State<GameState>>| match state.current() {
        GameState::None => ShouldRun::No,
        _ => ShouldRun::Yes,
    })
}

impl<T: StateData> Plugin for TowerDefensePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(RewardPlugin::new(GameState::Reward));
        app.add_state(GameState::None)
            .insert_resource(EndMenuState(self.end_menu_state.clone()))
            .insert_resource(GameStepTimer(Timer::from_seconds(0.1, true)))
            // Building systems
            .add_system_set(
                SystemSet::on_enter(GameState::Building)
                    .with_system(monster_spawn)
                    .with_system(added),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                in_game_system_set().with_system(Self::handle_reward),
            )
            .add_stage_after(
                CoreStage::Update,
                "Action",
                ActionStage::new(GameState::Fighting),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Building)
                    .with_system(select_tower)
                    .with_system(place_tower.before(select_tower))
                    .with_system(go)
                    .with_system(Self::start_wave),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                in_game_system_set()
                    .with_system(add_turn)
                    .with_system(remove_turn)
                    .with_system(removed)
                    .with_system(added)
                    .with_system(updated)
                    .with_system(add_health_bar)
                    .with_system(add_action)
                    .with_system(Self::game_over),
            )
            .add_system_set(SystemSet::on_exit(GameState::Building).with_system(grey_out))
            // Fighting systems
            .add_system_set(
                SystemSet::on_exit(GameState::Fighting)
                    .with_system(enable)
                    .with_system(return_to_start)
                    .with_system(reset_turn),
            )
            // Universal systems
            .add_system_set(
                SystemSet::on_enter(self.active_state.clone())
                    .with_system(Self::create_board)
                    .with_system(Self::add_start_ui),
            )
            .add_system_set(
                SystemSet::on_update(self.active_state.clone()).with_system(cursor_move),
            )
            .add_system_set(
                SystemSet::on_exit(self.active_state.clone())
                    .with_system(Self::clean_board)
                    .with_system(Self::clean_ui),
            )
            .add_event::<EnterBuildTarget>()
            .add_event::<HideBuildTarget>()
            .add_event::<TryBuild>()
            .add_event::<AttackEvent>()
            .add_event::<GameOver>()
            .add_event::<StartWave>()
            .add_event::<ActiveAction>()
            .add_event::<ActiveUnit>()
            .add_event::<Removed>();

        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
            // registering custom component to be able to edit it in inspector
        }
    }
}

pub struct UiRoot(pub Entity);

impl<T: StateData> TowerDefensePlugin<T> {
    fn handle_reward(
        mut commands: Commands,
        mut reward_evr: EventReader<Reward>,
        mut game_state: ResMut<State<GameState>>,
        board: ResMut<Board>,
        hero_sprites: Res<HeroSprites>,
    ) {
        for Reward(selected_reward) in reward_evr.iter() {
            game_state.set(GameState::Building).unwrap();
            for hero_type in selected_reward.iter() {
                let spawn = Coordinates::new(0, 0);
                add_hero(&mut commands, spawn, &board, &hero_sprites, *hero_type);
            }
        }
    }

    fn game_over(
        mut state: ResMut<State<T>>,
        mut game_state: ResMut<State<GameState>>,
        mut game_over_evr: EventReader<GameOver>,
        game_over_state: Res<EndMenuState<T>>,
    ) {
        for e in game_over_evr.iter() {
            match e.0 {
                false => {
                    game_state.set(GameState::Reward).unwrap();
                }
                true => {
                    game_state.set(GameState::None).unwrap();
                    state.push(game_over_state.0.clone()).unwrap();
                }
            }
        }
    }
    fn start_wave(
        mut game_state: ResMut<State<GameState>>,
        mut start_wave_evr: EventReader<StartWave>,
    ) {
        for _ in start_wave_evr.iter() {
            game_state.set(GameState::Fighting).unwrap();
        }
    }
    fn add_start_ui(mut commands: Commands, fonts: Res<Fonts>) {
        let ui_root = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::FlexEnd,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                color: UiColor(Color::rgba(0., 0., 0., 0.)),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        color: UiColor(Color::rgba(0., 0., 0., 0.)),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Px(400.),
                                        height: Val::Px(130.),
                                    },
                                    flex_direction: FlexDirection::Column,
                                    align_content: AlignContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                color: UiColor(Color::GRAY),
                                ..Default::default()
                            })
                            .insert(Go)
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        align_self: AlignSelf::Center,
                                        align_content: AlignContent::Center,
                                        ..Default::default()
                                    },
                                    text: Text {
                                        sections: vec![TextSection {
                                            value: "Go!".to_string(),
                                            style: TextStyle {
                                                font: fonts.get_handle(),
                                                font_size: 128.,
                                                ..Default::default()
                                            },
                                        }],
                                        alignment: TextAlignment::CENTER,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            });
                    });
            })
            .id();
        commands.insert_resource(UiRoot(ui_root));
    }

    fn clean_ui(mut commands: Commands, ui_root: Res<UiRoot>) {
        commands.entity(ui_root.0).despawn_recursive();
    }

    fn clean_board(mut commands: Commands, board: Res<Board>) {
        commands.entity(board.board.unwrap()).despawn_recursive();
    }

    fn spawn_ground(
        background: &mut ChildBuilder,
        board: &mut Board,
        spritesheets: &Res<GameSprites>,
    ) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let coordinate = Coordinates::new(x, y);
                let tile =
                    background
                        .spawn_bundle(TileBundle::new(
                            coordinate,
                            board.transform(&coordinate, 1.),
                        ))
                        .with_children(|parent| match board.tile_type(&coordinate) {
                            TileType::Grass => {
                                parent.spawn().insert(Name::new("Grass")).insert_bundle(
                                    spritesheets.grass(&coordinate, board.tile_size),
                                );
                            }
                            TileType::Dirt => {
                                parent
                                    .spawn()
                                    .insert(Name::new("Dirt"))
                                    .insert_bundle(spritesheets.dirt(board.tile_size));
                            }
                            TileType::Road => {
                                parent.spawn().insert(Name::new("Grass")).insert_bundle(
                                    spritesheets.grass(&coordinate, board.tile_size),
                                );
                                parent.spawn().insert(Name::new("Road")).insert_bundle(
                                    spritesheets.path(&coordinate, board, board.tile_size),
                                );
                            }
                            TileType::Start => {
                                parent.spawn().insert(Name::new("Grass")).insert_bundle(
                                    spritesheets.grass(&coordinate, board.tile_size),
                                );
                                parent.spawn().insert(Name::new("Road")).insert_bundle(
                                    spritesheets.path(&coordinate, board, board.tile_size),
                                );
                                parent
                                    .spawn()
                                    .insert(Name::new("Start"))
                                    .insert_bundle(spritesheets.spawn(board.tile_size));
                            }
                            TileType::Finish => {
                                parent.spawn().insert(Name::new("Grass")).insert_bundle(
                                    spritesheets.grass(&coordinate, board.tile_size),
                                );
                                parent.spawn().insert(Name::new("Road")).insert_bundle(
                                    spritesheets.path(&coordinate, board, board.tile_size),
                                );
                                parent
                                    .spawn()
                                    .insert(Name::new("Target"))
                                    .insert_bundle(spritesheets.end(board.tile_size));
                            }
                            TileType::Bench => {
                                parent
                                    .spawn()
                                    .insert(Name::new("Bench"))
                                    .insert_bundle(spritesheets.bench(board.tile_size));
                            }
                            TileType::Arrow => {
                                parent
                                    .spawn()
                                    .insert(Name::new("Arrow"))
                                    .insert_bundle(spritesheets.arrow(board.tile_size));
                            }
                            TileType::Result => {
                                parent
                                    .spawn()
                                    .insert(Name::new("Result"))
                                    .insert_bundle(spritesheets.result(board.tile_size));
                            }
                            TileType::Trainee => {
                                parent
                                    .spawn()
                                    .insert(Name::new("Trainee"))
                                    .insert_bundle(spritesheets.trainee(board.tile_size));
                            }
                            TileType::Trainer => {
                                parent
                                    .spawn()
                                    .insert(Name::new("Trainer"))
                                    .insert_bundle(spritesheets.trainer(board.tile_size));
                            }
                            _ => {}
                        })
                        .id();

                board.tiles.insert(coordinate, tile);
            }
        }
    }

    fn create_board(
        mut commands: Commands,
        spritesheets: Res<GameSprites>,
        mut game_state: ResMut<State<GameState>>,
        hero_sprites: Res<HeroSprites>,
    ) {
        game_state.set(GameState::Building).unwrap();
        let mut board = Board::new((20, 18), 32.);
        let board_position = board.board_offset();
        let board_entity = commands
            .spawn_bundle(BoardBundle::new(board_position))
            .with_children(|parent| {
                parent.spawn().insert(Cursor(None));
                parent.spawn().insert(Selected(None));
                parent.spawn().insert(TurnOrder(VecDeque::new(), 0));
                Self::spawn_ground(parent, &mut board, &spritesheets);
            })
            .id();
        board.board = Some(board_entity);
        commands.insert_resource(board);
        let mut board = Board::new((20, 18), 32.);
        board.board = Some(board_entity);
        add_hero(
            &mut commands,
            Coordinates::new(0, 0),
            &board,
            &hero_sprites,
            HeroType::Rogue,
        );
        add_hero(
            &mut commands,
            Coordinates::new(1, 0),
            &board,
            &hero_sprites,
            HeroType::Barbarian,
        );
        add_hero(
            &mut commands,
            Coordinates::new(2, 0),
            &board,
            &hero_sprites,
            HeroType::Paladin,
        );
    }
}
