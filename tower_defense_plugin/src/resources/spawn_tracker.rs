pub struct SpawnTracker {
    pub spawns_left: u32,
}

impl SpawnTracker {
    pub fn new() -> Self {
        SpawnTracker { spawns_left: 0 }
    }
}
