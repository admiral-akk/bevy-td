use bevy::{
    prelude::{
        Added, BuildChildren, Changed, Children, Color, Commands, DespawnRecursiveExt, Entity,
        EventReader, Query, Transform, Vec2, Visibility,
    },
    sprite::{Anchor, Sprite, SpriteBundle},
};

use crate::{
    components::{health::Health, health_bar::HealthBar},
    events::Attack,
};

pub fn damage(mut monsters: Query<&mut Health>, mut attack_evr: EventReader<Attack>) {
    for attack in attack_evr.iter() {
        if let Ok(mut health) = monsters.get_mut(attack.0) {
            health.health -= attack.1;
        }
    }
}

pub fn update_health_bar(
    changed: Query<(&Health, &Children), Changed<Health>>,
    mut health_bar: Query<(&HealthBar, &mut Sprite, &mut Visibility)>,
) {
    for (health, children) in changed.iter() {
        for child in children.iter() {
            if let Ok((health_bar, mut sprite, mut visibility)) = health_bar.get_mut(*child) {
                let size = sprite.custom_size.unwrap();
                sprite.custom_size = Some(Vec2::new(
                    health_bar.width * health.health as f32 / health.max as f32,
                    size.y,
                ));
                visibility.is_visible = health.health != health.max;
            }
        }
    }
}

pub fn add_health_bar(mut commands: Commands, changed: Query<Entity, Added<Health>>) {
    for changed in changed.iter() {
        let background = commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(26., 4.)),
                    anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                ..Default::default()
            })
            .id();
        let foreground = commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(26., 4.)),
                    anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                transform: Transform::from_xyz(-13., 20., 2.),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(HealthBar::new(26.))
            .add_child(background)
            .id();
        commands.entity(changed).add_child(foreground);
    }
}

pub fn death(mut commands: Commands, entities: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in entities.iter() {
        if health.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
