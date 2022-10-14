use assets_plugin::resources::{heroes::HeroSprites, monsters::MonsterSprites, sprites::Sprites};
use bevy::prelude::{Added, BuildChildren, Commands, Entity, Query, Res};

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
