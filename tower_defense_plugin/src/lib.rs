mod components;
mod events;
pub mod resources;
mod systems;

use assets_plugin::resources::fonts::Fonts;
use bevy::{
    ecs::schedule::{ShouldRun, StateData},
    prelude::*,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use components::{blueprint::Blueprint, coordinates::Coordinates, go::Go, tile::Tile};
use events::{
    Attack, EnterBuildTarget, GameOver, HideBuildTarget, Move, Spawn, StartWave, TryBuild,
};
use resources::{
    board::Board,
    build_tracker::BuildTracker,
    game_sprites::GameSprites,
    life_tracker::LifeTracker,
    spawn_timer::{AttackTimer, MoveTimer, SpawnTimer},
    spawn_tracker::SpawnTracker,
};
use systems::{
    blueprint::{enter_target, hide_blueprint},
    go::{enable, go, grey_out},
    health::{damage, death},
    input::{mouse_click_on_board, mouse_move_on_board},
    life::check_lives,
    monster::{monster_despawn, monster_move, monster_spawn},
    spawn::monster_tick,
    tower::{attack, try_build},
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

impl<T: StateData> Plugin for TowerDefensePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildTracker {
            target: None,
            blueprint: None,
        })
        .add_state(GameState::None)
        .insert_resource(EndMenuState(self.end_menu_state.clone()))
        .add_system_set(
            SystemSet::on_enter(self.active_state.clone())
                .with_system(Self::create_board)
                .with_system(Self::add_start_ui),
        )
        .add_system_set(
            SystemSet::on_update(self.active_state.clone())
                .with_run_criteria(
                    |game_state: Res<State<GameState>>| match game_state.current() {
                        GameState::Building => ShouldRun::Yes,
                        _ => ShouldRun::No,
                    },
                )
                .with_system(mouse_move_on_board)
                .with_system(mouse_click_on_board)
                .with_system(hide_blueprint)
                .with_system(enter_target)
                .with_system(try_build)
                .with_system(go)
                .with_system(Self::start_wave),
        )
        .add_system_set(SystemSet::on_exit(GameState::Fighting).with_system(enable))
        .add_system_set(SystemSet::on_exit(GameState::Building).with_system(grey_out))
        .add_system_set(
            SystemSet::on_update(self.active_state.clone())
                .with_run_criteria(
                    |game_state: Res<State<GameState>>| match game_state.current() {
                        GameState::Fighting => ShouldRun::Yes,
                        _ => ShouldRun::No,
                    },
                )
                .with_system(monster_tick)
                .with_system(monster_move)
                .with_system(attack)
                .with_system(monster_spawn)
                .with_system(monster_despawn)
                .with_system(damage)
                .with_system(death)
                .with_system(check_lives)
                .with_system(Self::game_over)
                .with_system(Self::wave_over),
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
        board: Res<Board>,
        life_tracker: Res<LifeTracker>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        if spawn_tracker.0 == 0 && board.monsters.len() == 0 && life_tracker.0 > 0 {
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
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::FlexEnd,
                    align_items: AlignItems::Center,

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
                                height: Val::Px(100.),
                            },
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
                            text: Text {
                                sections: vec![TextSection {
                                    value: "Go!".to_string(),
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
        commands.insert_resource(BuildTracker {
            target: None,
            blueprint: None,
        });
    }

    fn spawn_ground(
        background: &mut ChildBuilder,
        board: &mut Board,
        spritesheets: &Res<GameSprites>,
    ) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let coordinate = Coordinates::new(x, y);
                let tile = background
                    .spawn()
                    .insert(Name::new(format!("Tile {}, {}", x, y)))
                    .insert(Tile)
                    .insert(coordinate.clone())
                    .insert(GlobalTransform::default())
                    .insert(board.transform(&coordinate, 1.))
                    .insert_bundle(VisibilityBundle::default())
                    .with_children(|parent| {
                        parent
                            .spawn()
                            .insert(Name::new("Grass"))
                            .insert_bundle(spritesheets.grass(&coordinate, board.tile_size));
                        if board.is_path(&coordinate) {
                            parent.spawn().insert(Name::new("Road")).insert_bundle(
                                spritesheets.path(&coordinate, board, board.tile_size),
                            );
                        }
                        if board.is_start(&coordinate) {
                            parent
                                .spawn()
                                .insert(Name::new("Start"))
                                .insert_bundle(spritesheets.spawn(board.tile_size));
                        }
                        if board.is_end(&coordinate) {
                            parent
                                .spawn()
                                .insert(Name::new("Target"))
                                .insert_bundle(spritesheets.end(board.tile_size));
                        }
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
        let mut board = Board::new((16, 16), 32.);
        let board_position = board.board_offset();
        let board_entity = commands
            .spawn()
            .insert(Name::new("Game Map"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .insert_bundle(VisibilityBundle::default())
            .with_children(|parent| {
                parent
                    .spawn()
                    // .insert(Transform::from_xyz(map_size.x / 2., map_size.y / 2., 0.))
                    .insert(Name::new("Background"));
                Self::spawn_ground(parent, &mut board, &spritesheets);
                parent
                    .spawn()
                    .insert(Name::new("Blueprint"))
                    .insert(Blueprint)
                    .insert_bundle(TransformBundle::default())
                    .insert(Coordinates::default())
                    .insert_bundle(spritesheets.peasant(board.tile_size));
            })
            .id();
        board.board = Some(board_entity);
        commands.insert_resource(board);
        commands.insert_resource(SpawnTimer(Timer::from_seconds(2., true)));
        commands.insert_resource(MoveTimer(Timer::from_seconds(0.5, true)));
        commands.insert_resource(AttackTimer(Timer::from_seconds(0.5, true)));
        commands.insert_resource(LifeTracker(2));
        commands.insert_resource(SpawnTracker(0));
    }
}
