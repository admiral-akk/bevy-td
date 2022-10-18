use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::{debuffs::root::Root, movements::plan::Plan};

pub fn rooted(mut commands: Commands, rooted_units: Query<Entity, (With<Root>, With<Plan>)>) {
    for entity in rooted_units.iter() {
        commands.entity(entity).remove::<Plan>();
        commands.entity(entity).remove::<Root>();
    }
}
