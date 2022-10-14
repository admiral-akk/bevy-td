use bevy::prelude::Entity;

use crate::components::coordinates::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct EnterBuildTarget(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct HideBuildTarget;

#[derive(Debug, Copy, Clone)]
pub struct TryBuild;

#[derive(Debug, Copy, Clone)]
pub struct AttackEvent(pub Entity, pub i32);

#[derive(Debug, Copy, Clone)]
pub struct GameOver(pub bool);

#[derive(Debug, Copy, Clone)]
pub struct StartWave;

#[derive(Debug, Copy, Clone)]
pub struct ActiveUnit(pub Entity);

#[derive(Debug, Copy, Clone)]
pub struct Removed(pub Entity);

#[derive(Debug, Copy, Clone)]
pub struct ActiveAction(pub Entity);
