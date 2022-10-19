use assets_plugin::resources::{heroes::HeroSprites, monsters::MonsterSprites, sprites::Sprites};
use bevy::{
    prelude::{Added, BuildChildren, Commands, Entity, Query, Res, Transform, Vec3, With},
    sprite::{TextureAtlasSprite},
};

use crate::components::{hero::Hero, monster::Monster};

pub fn monster_added(
    mut commands: Commands,
    added: Query<(Entity, &Monster), Added<Monster>>,
    sprites: Res<MonsterSprites>,
) {
    for (entity, monster) in added.iter() {
        let sprite = commands
            .spawn_bundle(sprites.fetch_sprite_sheet(monster.0))
            .id();
        commands.entity(entity).add_child(sprite);
    }
}

pub fn hero_added(
    mut commands: Commands,
    added: Query<(Entity, &Hero), Added<Hero>>,
    sprites: Res<HeroSprites>,
) {
    for (entity, hero) in added.iter() {
        let sprite = commands
            .spawn_bundle(sprites.fetch_sprite_sheet(hero.0))
            .id();
        commands.entity(entity).add_child(sprite);
    }
}

const STEP_SIZE: f32 = 5.;

pub fn move_sprite_back(mut sprites: Query<&mut Transform, With<TextureAtlasSprite>>) {
    for mut transform in sprites.iter_mut() {
        let pos = transform.translation;
        if pos.length() < STEP_SIZE {
            *transform = transform.with_translation(Vec3::ZERO);
        } else {
            *transform = transform.with_translation(pos - pos.normalize() * STEP_SIZE);
        }
    }
}
