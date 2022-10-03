mod bundles;
mod components;
mod entities;
mod events;
pub mod resources;
mod systems;

use assets_plugin::resources::fonts::Fonts;
use bevy::{ecs::schedule::StateData, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use bundles::{board_bundle::BoardBundle, tile_bundle::TileBundle};
use components::{
    coordinates::Coordinates, cursor::Cursor, go::Go, lives::Lives, monster::Monster,
    selected::Selected,
};

use events::{
    Attack, EnterBuildTarget, GameOver, HideBuildTarget, Move, Spawn, StartWave, TryBuild,
};
use resources::{
    board::{Board, TileType},
    game_sprites::GameSprites,
    life_tracker::LifeTracker,
    spawn_timer::{AttackTimer, MoveTimer, SpawnTimer},
    spawn_tracker::SpawnTracker,
};
use systems::{
    coordinates::{
        remove_monsters, remove_towers, update_monsters, update_towers, update_transform,
    },
    cursor::cursor_move,
    go::{enable, go, grey_out},
    health::{damage, death},
    life::{check_lives, update_lives},
    monster::{monster_despawn, monster_move, monster_spawn},
    reward::spawn_reward,
    selected::{place_tower, select_tower},
    spawn::monster_tick,
    tower::attack,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum GameStages {
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    /// everything that moves things (works with transforms)
    Movement,
    /// systems that update the world map
    Map,
}

impl<T: StateData> Plugin for TowerDefensePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::None)
            .insert_resource(EndMenuState(self.end_menu_state.clone()))
            .insert_resource(SpawnTimer(Timer::from_seconds(2., true)))
            .insert_resource(MoveTimer(Timer::from_seconds(0.5, true)))
            .insert_resource(AttackTimer(Timer::from_seconds(0.5, true)))
            // Building systems
            .add_system_set(SystemSet::on_enter(GameState::Building).with_system(spawn_reward))
            .add_system_set(
                SystemSet::on_update(GameState::Building)
                    .with_system(select_tower)
                    .with_system(place_tower.before(select_tower))
                    .with_system(remove_towers)
                    .with_system(go)
                    .with_system(Self::start_wave),
            )
            .add_system_set(SystemSet::on_exit(GameState::Building).with_system(grey_out))
            // Fighting systems
            .add_system_set(
                SystemSet::on_update(GameState::Fighting)
                    .with_system(monster_tick)
                    .with_system(attack.after(monster_tick))
                    .with_system(damage.after(attack))
                    .with_system(death.after(damage))
                    .with_system(monster_move.after(death))
                    .with_system(monster_despawn.after(monster_move))
                    .with_system(remove_monsters.after(monster_despawn))
                    .with_system(check_lives.after(remove_monsters))
                    .with_system(Self::game_over.after(check_lives))
                    .with_system(Self::wave_over.after(Self::game_over))
                    .with_system(monster_spawn.after(Self::wave_over))
                    .with_system(update_lives.after(monster_spawn)),
            )
            .add_system_set(SystemSet::on_exit(GameState::Fighting).with_system(enable))
            // Universal systems
            .add_system_set(
                SystemSet::on_enter(self.active_state.clone())
                    .with_system(Self::create_board)
                    .with_system(Self::add_start_ui),
            )
            .add_system_set(
                SystemSet::on_update(self.active_state.clone())
                    .with_system(cursor_move)
                    .with_system(update_transform)
                    .with_system(update_towers)
                    .with_system(update_monsters),
            )
            .add_system_set(
                SystemSet::on_exit(self.active_state.clone())
                    .with_system(Self::clean_board)
                    .with_system(Self::clean_ui),
            )
            .add_event::<EnterBuildTarget>()
            .add_event::<HideBuildTarget>()
            .add_event::<TryBuild>()
            .add_event::<Spawn>()
            .add_event::<Attack>()
            .add_event::<Move>()
            .add_event::<GameOver>()
            .add_event::<StartWave>();

        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
            // registering custom component to be able to edit it in inspector
        }
    }
}

pub struct UiRoot(pub Entity);

impl<T: StateData> TowerDefensePlugin<T> {
    fn start_wave(
        mut game_state: ResMut<State<GameState>>,
        mut start_wave_evr: EventReader<StartWave>,
        mut spawn_tracker: ResMut<SpawnTracker>,
    ) {
        for _ in start_wave_evr.iter() {
            game_state.set(GameState::Fighting).unwrap();
            spawn_tracker.0 = 5;
        }
    }

    fn wave_over(
        spawn_tracker: Res<SpawnTracker>,
        life_tracker: Res<LifeTracker>,
        mut game_state: ResMut<State<GameState>>,
        monsters: Query<With<Monster>>,
    ) {
        if spawn_tracker.0 == 0 && monsters.is_empty() && life_tracker.0 > 0 {
            game_state.set(GameState::Building).unwrap();
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
                            .spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Lives: 2".to_string(),
                                        style: TextStyle {
                                            font: fonts.get_handle(),
                                            font_size: 128.,
                                            ..Default::default()
                                        },
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Lives);
                    });
            })
            .id();
        commands.insert_resource(UiRoot(ui_root));
    }

    fn clean_ui(mut commands: Commands, ui_root: Res<UiRoot>) {
        commands.entity(ui_root.0).despawn_recursive();
    }

    fn game_over(
        mut state: ResMut<State<T>>,
        mut game_state: ResMut<State<GameState>>,
        mut game_over_evr: EventReader<GameOver>,
        game_over_state: Res<EndMenuState<T>>,
    ) {
        for _ in game_over_evr.iter() {
            game_state.set(GameState::None).unwrap();
            state.push(game_over_state.0.clone()).unwrap();
        }
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
                Self::spawn_ground(parent, &mut board, &spritesheets);
            })
            .id();
        board.board = Some(board_entity);
        commands.insert_resource(board);
        commands.insert_resource(LifeTracker(2));
        commands.insert_resource(SpawnTracker(0));
    }
}
