mod board;
mod components;
mod resources;

use bevy::{prelude::*, window::WindowDescriptor};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use board::game_map::{self, GameMap};
use components::coordinates::Coordinates;
use resources::board::Board;
pub struct TowerDefensePlugin {}

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
            // registering custom component to be able to edit it in inspector
        }
    }
}

impl TowerDefensePlugin {
    fn spawn_ground(background: &mut ChildBuilder, board: &mut Board, size: f32) {
        for y in 0..board.height() {
            for x in 0..board.width() {
                let coordinate = Coordinates::new(x, y);
                let tile = background
                    .spawn()
                    .insert(Name::new(format!("Tile {}, {}", x, y)))
                    .insert(coordinate.clone())
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0., 0., 1.),
                            custom_size: Some(Vec2::new(size - 1., size - 1.)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            x as f32 * size + size / 2.,
                            y as f32 * size + size / 2.,
                            1.,
                        ),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        if board.is_path(&coordinate) {
                            parent
                                .spawn()
                                .insert(Name::new("Road"))
                                .insert_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::rgb(1., 1., 1.),
                                        custom_size: Some(Vec2::new(size, size)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(0., 0., 1.),
                                    ..Default::default()
                                });
                        }
                    })
                    .id();

                board.tiles.insert(coordinate, tile);
            }
        }
    }

    fn create_board(mut commands: Commands, window: Res<WindowDescriptor>) {
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
                Self::spawn_ground(parent, &mut board, tile_size);
            })
            .id();
        commands.insert_resource(board);
    }
}
