use assets_plugin::resources::{fonts::Fonts, heroes::HeroType};
use bevy::{
    prelude::{
        BuildChildren, Color, Commands, DespawnRecursiveExt, Entity, NodeBundle, Query, Res, With,
    },
    ui::{
        AlignContent, AlignItems, AlignSelf, FlexDirection, JustifyContent, PositionType, Size,
        Style, UiColor, UiRect, Val,
    },
};

use bevy::prelude::Component;

use super::button::reward_button;

#[derive(Component)]
pub struct RewardUi;

pub fn remove_reward_ui(mut commands: Commands, ui: Query<Entity, With<RewardUi>>) {
    for ui in ui.iter() {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_root(commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::FlexEnd,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(100.)),
                ..Default::default()
            },
            color: UiColor(Color::rgba(0., 0., 0., 0.)),

            ..Default::default()
        })
        .insert(RewardUi)
        .id()
}

fn spawn_background(commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::FlexEnd,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: UiColor(Color::rgba(0., 0., 0., 0.5)),
            ..Default::default()
        })
        .id()
}

pub fn add_reward_ui(mut commands: Commands, fonts: Res<Fonts>) {
    let root = spawn_root(&mut commands);

    let background = spawn_background(&mut commands);

    let reward_1 = reward_button(&mut commands, &fonts, HeroType::Rogue);
    let reward_2 = reward_button(&mut commands, &fonts, HeroType::Barbarian);
    let reward_3 = reward_button(&mut commands, &fonts, HeroType::Paladin);

    commands.entity(root).add_child(background);
    commands
        .entity(background)
        .push_children(&[reward_1, reward_2, reward_3]);
}
