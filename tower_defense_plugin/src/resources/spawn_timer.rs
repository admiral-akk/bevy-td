use bevy::time::Timer;

pub struct SpawnTimer(pub Timer);
pub struct MoveTimer(pub Timer);
pub struct AttackTimer(pub Timer);
pub struct GameTickTimer(pub Timer);
