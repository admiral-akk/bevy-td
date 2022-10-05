pub struct SpawnTracker {
    pub spawns_left: u32,
    tick_to_spawn: u32,
    ticks_remaining: u32,
}

impl SpawnTracker {
    pub fn new(tick_to_spawn: u32) -> Self {
        SpawnTracker {
            spawns_left: 0,
            tick_to_spawn,
            ticks_remaining: 0,
        }
    }
}
