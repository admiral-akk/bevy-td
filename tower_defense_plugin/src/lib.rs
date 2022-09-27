mod board;
mod components;
pub mod resources;

use bevy::{ecs::schedule::StateData, prelude::*, window::WindowDescriptor};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use board::game_map::GameMap;
use components::coordinates::Coordinates;
use resources::{board::Board, game_sprites::GameSprites};

pub struct TowerDefensePlugin<T> {
    pub active_state: T,
}

impl<T: StateData> Plugin for TowerDefensePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(self.active_state.clone()).with_system(Self::create_board),
        );
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
        size: f32,
        spritesheets: Res<GameSprites>,
    ) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let coordinate = Coordinates::new(x, y);
                let tile = background
                    .spawn()
                    .insert(Name::new(format!("Tile {}, {}", x, y)))
                    .insert(coordinate.clone())
                    .insert_bundle(spritesheets.grass(&coordinate, size))
                    .with_children(|parent| {
                        if board.is_path(&coordinate) {
                            parent
                                .spawn()
                                .insert(Name::new("Road"))
                                .insert_bundle(spritesheets.path(&coordinate, board, size));
                        }
                        if board.is_start(&coordinate) {
                            parent
                                .spawn()
                                .insert(Name::new("Start"))
                                .insert_bundle(spritesheets.spawn(size));
                        }
                        if board.is_end(&coordinate) {
                            parent
                                .spawn()
                                .insert(Name::new("Target"))
                                .insert_bundle(spritesheets.end(size));
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
        let tile_size = 32.;
        let map_size = Vec2::new(
            map.width() as f32 * tile_size,
            map.height() as f32 * tile_size,
        );
        let board_position = Vec3::new(-(map_size.x / 2.), -(map_size.y / 2.), 0.);
        let mut board = Board::new();
        let board_entity = commands
            .spawn()
            .insert(Name::new("Game Map"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .insert_bundle(VisibilityBundle::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1., 0., 1.),
                            custom_size: Some(map_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(map_size.x / 2., map_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
                Self::spawn_ground(parent, &mut board, tile_size, spritesheets);
            })
            .id();
        commands.insert_resource(board);
    }
}
