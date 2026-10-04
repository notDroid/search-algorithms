use std::collections::HashMap;
use std::hash::Hash;

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
        HashMap::get(self, &key).copied()
    }

    #[inline]
    fn put(&mut self, key: Self::K, value: Self::V) {
        self.insert(key, value);
    }
}
