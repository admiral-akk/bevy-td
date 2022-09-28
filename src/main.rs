use assets_plugin::AssetsPlugin;
use bevy::prelude::*;

use bevy::render::texture::ImageSettings;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use end_menu_plugin::EndMenuPlugin;
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
        end_menu_state: GameState::GameOver,
    })
    .add_plugin(StartMenuPlugin {
        active_state: GameState::Start,
        in_game_state: GameState::InGame,
    })
    .add_plugin(EndMenuPlugin {
        active_state: GameState::GameOver,
        in_game_state: GameState::InGame,
        start_menu_state: GameState::Start,
    })
    .add_plugin(AssetsPlugin)
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

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(Camera2dBundle::default());
}
