use crate::components::coordinates::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct EnterBuildTarget(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct HideBuildTarget;

#[derive(Debug, Copy, Clone)]
pub struct TryBuild;
