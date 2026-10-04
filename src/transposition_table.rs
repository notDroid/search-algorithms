use crate::kv::KVStore;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct KVWithReplacement<K, V> {
    n: usize,
    array: Vec<Option<(K, V)>>,
}

impl<K, V> KVWithReplacement<K, V>
where
    K: Hash + Eq + PartialEq + Copy,
    V: Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            array: vec![None; n],
        }
    }

    #[inline]
    pub fn hash(&self, key: K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.n
    }
}

impl<K, V> KVStore for KVWithReplacement<K, V>
where
    K: Hash + Eq + PartialEq + Copy,
    V: Copy,
{
    type K = K;
    type V = V;

    #[inline]
    fn get(&self, key: Self::K) -> Option<Self::V> {
        let hash = self.hash(key);
        let (found_key, value) = (*self.array.get(hash)?)?;

        if found_key == key { Some(value) } else { None }
    }

    #[inline]
    fn put(&mut self, key: Self::K, value: Self::V) {
        let hash = self.hash(key);
        self.array[hash] = Some((key, value));
    }
}
