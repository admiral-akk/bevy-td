use bevy::{
    prelude::{AssetServer, Assets, Handle, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

pub struct PathSprites {
    pub path_atlas_handle: Option<Handle<TextureAtlas>>,
}
