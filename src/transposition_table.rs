use crate::kv::KVStore;
use std::hash::{BuildHasher, Hash, RandomState};

pub struct KVWithReplacement<K, V, S: BuildHasher = RandomState> {
    n: usize,
    array: Vec<Option<(K, V)>>,
    hasher_builder: S,
}

impl<K, V, S: BuildHasher> KVWithReplacement<K, V, S>
where
    K: Hash + Eq + PartialEq + Copy,
    V: Copy,
{
    pub fn new_with_hasher(n: usize, hasher_builder: S) -> Self {
        Self {
            n,
            array: vec![None; n],
            hasher_builder,
        }
    }

    #[inline]
    pub fn hash(&self, key: K) -> usize {
        (self.hasher_builder.hash_one(key) as usize) % self.n
    }
}

impl<K, V> KVWithReplacement<K, V, RandomState>
where
    K: Hash + Eq + PartialEq + Copy,
    V: Copy,
{
    pub fn new(n: usize) -> Self {
        Self::new_with_hasher(n, RandomState::new())
    }
}

impl<K, V, S> KVStore for KVWithReplacement<K, V, S>
where
    K: Hash + Eq + PartialEq + Copy,
    V: Copy,
    S: BuildHasher,
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
