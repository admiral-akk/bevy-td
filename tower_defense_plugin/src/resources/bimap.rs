use std::hash::Hash;

use bevy::utils::HashMap;

pub struct BiMap<K: Eq + Hash, V: Eq + Hash> {
    kv: HashMap<K, V>,
    vk: HashMap<V, K>,
}
impl<K: Eq + Hash + Copy, V: Eq + Hash + Copy> BiMap<K, V> {
    pub fn new() -> Self {
        Self {
            kv: HashMap::new(),
            vk: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.kv.len()
    }

    pub fn update_key(&mut self, v: &V, new_k: K) {
        if let Some(old_k) = self.vk.get(v) {
            self.kv.remove(old_k);
        }
        self.vk.remove(v);
        self.insert(new_k, *v);
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.kv.insert(k, v);
        self.vk.insert(v, k);
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.kv.contains_key(&k)
    }

    pub fn contains_value(&self, v: &V) -> bool {
        self.vk.contains_key(&v)
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let v = self.kv.remove(&k);
        if let Some(v) = v {
            self.vk.remove(&v);
        }
        v
    }

    pub fn remove_value(&mut self, v: &V) -> Option<K> {
        let k = self.vk.remove(&v);
        if let Some(k) = k {
            self.kv.remove(&k);
        }
        k
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.kv.get(&k)
    }

    pub fn get_key(&self, v: &V) -> Option<&K> {
        self.vk.get(&v)
    }
}
