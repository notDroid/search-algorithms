use std::hash::Hash;
use std::collections::HashMap;

pub trait StateKey {
    type Key: Hash + Eq + Copy;

    fn key(&self) -> Self::Key;
}

pub trait KVStore {
    type K: Hash + Eq + Copy;
    type V: Copy;

    fn get(&self, key: Self::K) -> Option<Self::V>;
    fn put(&mut self, key: Self::K, value: Self::V);
}

impl<K, V> KVStore for HashMap<K, V> 
where
    K: Hash + Eq + PartialEq + Copy,
    V: Copy,
{
    type K = K;
    type V = V;

    #[inline]
    fn get(&self, key: Self::K) -> Option<Self::V> {
        HashMap::get(self, &key).map(|v| *v)
    }

    #[inline]
    fn put(&mut self, key: Self::K, value: Self::V) {
        self.insert(key, value);
    }
}