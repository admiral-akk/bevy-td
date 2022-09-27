mod components;
mod events;
pub mod resources;
mod systems;

use bevy::{ecs::schedule::StateData, prelude::*, window::WindowDescriptor};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use components::{blueprint::Blueprint, coordinates::Coordinates, tile::Tile};
use events::{EnterBuildTarget, HideBuildTarget, Move, Spawn, TryBuild};
use resources::{
    board::Board,
    build_tracker::BuildTracker,
    game_sprites::GameSprites,
    spawn_timer::{AttackTimer, MoveTimer, SpawnTimer},
};
use systems::{
    blueprint::{enter_target, hide_blueprint},
    input::{mouse_click_on_board, mouse_move_on_board},
    monster::{monster_despawn, monster_move, monster_spawn},
    spawn::monster_tick,
    tower::try_build,
};

pub struct TowerDefensePlugin<T> {
    pub active_state: T,
}

impl<T: StateData> Plugin for TowerDefensePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildTracker {
            target: None,
            blueprint: None,
        })
        .add_system_set(
            SystemSet::on_enter(self.active_state.clone()).with_system(Self::create_board),
        )
        .add_system_set(
            SystemSet::on_update(self.active_state.clone())
                .with_system(monster_tick)
                .with_system(monster_move)
                .with_system(mouse_move_on_board)
                .with_system(mouse_click_on_board)
                .with_system(hide_blueprint)
                .with_system(enter_target)
                .with_system(monster_spawn)
                .with_system(monster_despawn)
                .with_system(try_build),
        )
        .add_event::<EnterBuildTarget>()
        .add_event::<HideBuildTarget>()
        .add_event::<TryBuild>()
        .add_event::<Spawn>()
        .add_event::<Move>();
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
            // registering custom component to be able to edit it in inspector
        }
    }
}

impl<T> TowerDefensePlugin<T> {
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
        _window: Res<WindowDescriptor>,
        spritesheets: Res<GameSprites>,
    ) {
        let mut board = Board::new((16, 16), 32.);
        let _map_size = board.board_size();
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
        commands.insert_resource(SpawnTimer(Timer::from_seconds(4., true)));
        commands.insert_resource(MoveTimer(Timer::from_seconds(1., true)));
        commands.insert_resource(AttackTimer(Timer::from_seconds(1., true)));
    }
}
