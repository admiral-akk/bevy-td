use bevy::{
    log,
    prelude::{BuildChildren, Commands, EventReader, Name, Res, ResMut},
    transform::TransformBundle,
};

use crate::{
    components::tower::Tower,
    events::TryBuild,
    resources::{board::Board, build_tracker::BuildTracker, game_sprites::GameSprites},
};

pub fn attack() {}

pub fn try_build(
    mut commands: Commands,
    mut board: ResMut<Board>,
    spritesheet: Res<GameSprites>,
    build_tracker: Res<BuildTracker>,
    mut build_evr: EventReader<TryBuild>,
) {
    for _ in build_evr.iter() {
        if let Some(coord) = build_tracker.target {
            commands
                .entity(board.board.unwrap())
                .with_children(|parent| {
                    let tower = parent
                        .spawn()
                        .insert(Name::new("Tower"))
                        .insert(Tower)
                        .insert(coord.clone())
                        .insert_bundle(spritesheet.peasant(board.tile_size))
                        .insert_bundle(TransformBundle {
                            local: board.transform(&coord, 4.),
                            ..Default::default()
                        })
                        .id();
                    board.towers.insert(coord, tower);
                });
        }
    }
}
