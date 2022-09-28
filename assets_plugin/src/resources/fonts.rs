use bevy::{prelude::Handle, text::Font};

pub struct Fonts {
    font: Option<Handle<Font>>,
}

impl Fonts {
    pub fn init() -> Self {
        Fonts { font: None }
    }

    pub fn update_handle(&mut self, font: Handle<Font>) {
        self.font = Some(font);
    }

    pub fn get_handle(&self) -> Handle<Font> {
        self.font.as_ref().unwrap().clone()
    }
}
