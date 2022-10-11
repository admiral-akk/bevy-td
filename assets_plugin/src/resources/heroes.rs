use std::collections::HashMap;

use bevy::{
    prelude::{AssetServer, Assets, Commands, Handle, Image, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Hash, Eq, PartialEq, EnumIter, Display)]
pub enum HeroTypes {
    Shepard,
    Assassin,
    Rogue,
}

impl HeroTypes {
    fn to_path(&self) -> String {
        format!("spritesheets/heroes/{}.png", self.to_string())
    }
}

pub struct HeroSprites {
    handles: HashMap<HeroTypes, Handle<TextureAtlas>>,
}

impl HeroSprites {
    pub fn instantiate(
        mut commands: Commands,
        server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let mut handles = HashMap::new();
        for hero in HeroTypes::iter() {
            let texture_handle: Handle<Image> = server.load(&hero.to_path());
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            handles.insert(hero, texture_atlas_handle);
        }
        commands.insert_resource(HeroSprites { handles })
    }

    pub fn fetch_sprite_sheet(&self, hero_type: HeroTypes) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture_atlas: self.handles.get(&hero_type).unwrap().clone(),
            transform: Transform::from_scale(Vec3::splat(12.)),
            ..Default::default()
        }
    }
}
