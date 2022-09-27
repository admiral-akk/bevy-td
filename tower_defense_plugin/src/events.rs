use crate::components::coordinates::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct EnterBuildTarget(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct ExitBuildTarget(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct TryBuild;
