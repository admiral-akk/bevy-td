use bevy::log;
use bevy::prelude::*;

use bevy::render::texture::ImageSettings;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::resources::board_assets::BoardAssets;
use board_plugin::resources::board_assets::SpriteMaterial;
use board_plugin::resources::board_options::BoardOptions;
use start_menu_plugin::StartMenuPlugin;
use tower_defense_plugin::resources::game_sprites::GameSprites;
use tower_defense_plugin::TowerDefensePlugin;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
    Paused,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    InGame,
    Start,
    GameOver,
}
fn main() {
    let mut app = App::new();
    // Window setup
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Mine Sweeper!".to_string(),
            width: 700.,
            height: 900.,
            ..Default::default()
        })
        // Bevy default plugins
        .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());

    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    app.add_plugin(TowerDefensePlugin {
        active_state: GameState::InGame,
    })
    .add_plugin(StartMenuPlugin {
        active_state: GameState::Start,
        in_game_state: GameState::InGame,
    })
    .insert_resource(GameSprites::init())
    .add_startup_system(load_resources)
    .add_state(GameState::Start)
    .run();
    // app.add_state(AppState::Out)
    //     .add_plugin(BoardPlugin {
    //         running_state: AppState::InGame,
    //     })
    //     .add_system(state_handler);
    // Run the app
}
fn load_resources(
    mut path_sprites: ResMut<GameSprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("spritesheets/rts_medival/tilemap_packed.png");
    let path_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(15.0, 15.0),
        23,
        9,
        Vec2::new(1., 1.),
        Vec2::new(1., 1.),
    );
    path_sprites.update_handle(texture_atlases.add(path_atlas));
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        safe_start: true,
        ..Default::default()
    });
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    });
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        } else if state.current() == &AppState::InGame {
            state.restart().unwrap();
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pasuing detected");
        if state.current() != &AppState::Paused {
            log::info!("pausing game");
            state.push(AppState::Paused).unwrap();
        } else {
            state.pop().unwrap();
        }
    }
}
