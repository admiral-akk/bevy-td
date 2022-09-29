use bevy::prelude::Entity;

use crate::components::coordinates::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct EnterBuildTarget(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct HideBuildTarget;

#[derive(Debug, Copy, Clone)]
pub struct TryBuild;

#[derive(Debug, Copy, Clone)]
pub struct Spawn;

#[derive(Debug, Copy, Clone)]
pub struct Move;

#[derive(Debug, Copy, Clone)]
pub struct Attack(pub Entity, pub u32);

#[derive(Debug, Copy, Clone)]
pub struct GameOver;

#[derive(Debug, Copy, Clone)]
pub struct StartWave;
