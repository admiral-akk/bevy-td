mod bundles;
mod components;
mod entities;
mod events;
mod plugins;
pub mod resources;
mod systems;

use std::collections::VecDeque;

use assets_plugin::resources::fonts::Fonts;
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

use entities::towers::{get_tower, TowerType};
use events::{
    ActiveUnit, Attack, EnterBuildTarget, GameOver, HideBuildTarget, Removed, StartWave, TryBuild,
};
use plugins::{events::Reward, reward_plugin::RewardPlugin};
use resources::{
    board::{Board, TileType},
    game_sprites::GameSprites,
    game_step_timer::GameStepTimer,
};
use systems::{
    attack::attack,
    coordinates::{added, removed, return_to_start, updated},
    cursor::cursor_move,
    go::{enable, go, grey_out},
    health::{add_health_bar, damage, death, update_health_bar},
    life::check_units,
    movement::movement,
    selected::{place_tower, select_tower},
    spawn_wave::monster_spawn,
    turn_order::{add_turn, remove_turn, tick_active},
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

#[derive(Copy, Clone, StageLabel)]
pub enum GameStage {
    Tick,
    Move,
    Attack,
    ResolveAttack,
    CheckEnd,
    CleanUp,
}

impl GameStage {
    const STAGES: [GameStage; 6] = [
        GameStage::Tick,
        GameStage::Attack,
        GameStage::ResolveAttack,
        GameStage::Move,
        GameStage::CheckEnd,
        GameStage::CleanUp,
    ];

    pub fn add_stages(app: &mut App) {
        for i in 0..GameStage::STAGES.len() {
            if i == 0 {
                app.add_stage_after(
                    CoreStage::Update,
                    GameStage::STAGES[i],
                    SystemStage::single_threaded(),
                );
            } else {
                app.add_stage_after(
                    GameStage::STAGES[i - 1],
                    GameStage::STAGES[i],
                    SystemStage::single_threaded(),
                );
            }
        }
    }
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
        GameStage::add_stages(app);
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
            .add_system_set(
                SystemSet::on_update(GameState::Building)
                    .with_system(select_tower)
                    .with_system(place_tower.before(select_tower))
                    .with_system(go)
                    .with_system(Self::start_wave),
            )
            .add_system_set(SystemSet::on_exit(GameState::Building).with_system(grey_out))
            // Fighting systems
            .add_system_set_to_stage(
                GameStage::Tick,
                fighting_system_set().with_system(tick_active),
            )
            .add_system_set_to_stage(
                GameStage::Move,
                SystemSet::new()
                    .with_run_criteria(is_fighting)
                    .with_system(movement),
            )
            .add_system_set_to_stage(GameStage::Attack, fighting_system_set().with_system(attack))
            .add_system_set_to_stage(
                GameStage::ResolveAttack,
                fighting_system_set()
                    .with_system(damage)
                    .with_system(death.after(damage))
                    .with_system(update_health_bar.after(damage)),
            )
            .add_system_set_to_stage(
                GameStage::CleanUp,
                in_game_system_set()
                    .with_system(add_turn)
                    .with_system(remove_turn)
                    .with_system(removed)
                    .with_system(added)
                    .with_system(updated)
                    .with_system(add_health_bar),
            )
            .add_system_set_to_stage(
                GameStage::CheckEnd,
                fighting_system_set()
                    .with_system(check_units)
                    .with_system(Self::game_over.after(check_units)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Fighting)
                    .with_system(enable)
                    .with_system(return_to_start),
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
            .add_event::<Attack>()
            .add_event::<GameOver>()
            .add_event::<StartWave>()
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
        mut board: ResMut<Board>,
        spritesheets: Res<GameSprites>,
    ) {
        for selected_reward in reward_evr.iter() {
            game_state.set(GameState::Building).unwrap();
            for tower in selected_reward.0.iter() {
                let spawn = Coordinates::new(0, 0);
                get_tower(&mut commands, &mut board, &spawn, &spritesheets, *tower);
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
    ) {
        game_state.set(GameState::Building).unwrap();
        let mut board = Board::new((20, 18), 32.);
        let board_position = board.board_offset();
        let board_entity = commands
            .spawn_bundle(BoardBundle::new(board_position))
            .with_children(|parent| {
                parent.spawn().insert(Cursor(None));
                parent.spawn().insert(Selected(None));
                parent.spawn().insert(TurnOrder(VecDeque::new()));
                Self::spawn_ground(parent, &mut board, &spritesheets);
            })
            .id();
        board.board = Some(board_entity);
        let spawn = Coordinates::new(0, 0);
        get_tower(
            &mut commands,
            &board,
            &spawn,
            &spritesheets,
            TowerType::Guard,
        );
        commands.insert_resource(board);
    }
}
