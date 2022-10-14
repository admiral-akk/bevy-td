use bevy::sprite::SpriteSheetBundle;

pub trait Sprites<T> {
    fn fetch_sprite_sheet(&self, sprite_type: T) -> SpriteSheetBundle;
}
