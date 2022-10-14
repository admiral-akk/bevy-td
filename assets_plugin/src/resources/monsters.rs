use std::collections::HashMap;

use bevy::{
    prelude::{AssetServer, Assets, Commands, Handle, Image, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Hash, Eq, PartialEq, EnumIter, Display)]
pub enum MonsterType {
    Bat,
    Jelly,
    Treant,
    Zombie,
}

impl MonsterType {
    fn to_path(&self) -> String {
        format!("spritesheets/monsters/{}.png", self.to_string())
    }
}

pub struct MonsterSprites {
    handles: HashMap<MonsterType, Handle<TextureAtlas>>,
}

impl MonsterSprites {
    pub fn instantiate(
        mut commands: Commands,
        server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let mut handles = HashMap::new();
        for monster in MonsterType::iter() {
            let texture_handle: Handle<Image> = server.load(&monster.to_path());
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            handles.insert(monster, texture_atlas_handle);
        }
        commands.insert_resource(MonsterSprites { handles })
    }

    pub fn fetch_sprite_sheet(&self, monster: MonsterType) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture_atlas: self.handles.get(&monster).unwrap().clone(),
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..Default::default()
        }
    }
}
