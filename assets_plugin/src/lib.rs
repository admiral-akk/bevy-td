#![feature(adt_const_params)]
pub mod resources;
use bevy::prelude::{AssetServer, Plugin, Res, ResMut};
use resources::{fonts::Fonts, heroes::HeroSprites};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Fonts::init())
            .add_startup_system(Self::load_fonts)
            .add_startup_system(HeroSprites::instantiate);
    }
}

impl AssetsPlugin {
    fn load_fonts(mut fonts: ResMut<Fonts>, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/pixeled.ttf");
        fonts.update_handle(font);
    }
}
