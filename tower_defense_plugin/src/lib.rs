mod board;
mod components;
mod events;
pub mod resources;
mod systems;

use bevy::{ecs::schedule::StateData, prelude::*, window::WindowDescriptor};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use board::game_map::GameMap;
use components::coordinates::Coordinates;
use events::{EnterBuildTarget, ExitBuildTarget, TryBuild};
use resources::{board::Board, build_tracker::BuildTracker, game_sprites::GameSprites};
use systems::{
    blueprint::{enter_target, exit_target},
    input::{mouse_click_on_board, mouse_move_on_board},
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
                .with_system(mouse_move_on_board)
                .with_system(mouse_click_on_board)
                .with_system(exit_target)
                .with_system(enter_target),
        )
        .add_event::<EnterBuildTarget>()
        .add_event::<ExitBuildTarget>()
        .add_event::<TryBuild>();
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
        spritesheets: Res<GameSprites>,
    ) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let coordinate = Coordinates::new(x, y);
                let tile = background
                    .spawn()
                    .insert(Name::new(format!("Tile {}, {}", x, y)))
                    .insert(coordinate.clone())
                    .insert_bundle(spritesheets.grass(&coordinate, board.tile_size))
                    .with_children(|parent| {
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
        window: Res<WindowDescriptor>,
        spritesheets: Res<GameSprites>,
    ) {
        let mut map = GameMap::empty(16, 16, Coordinates::new(2, 8), Coordinates::new(12, 8));

        let mut board = Board::new((16, 16), 32.);
        let map_size = board.board_size();
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
                Self::spawn_ground(parent, &mut board, spritesheets);
            })
            .id();
        commands.insert_resource(board);
    }
}
