use alloc::vec::*;
use core::hash::*;

const INITIAL_CAPACITY: usize = 16;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

struct MyHasher(u64);

impl MyHasher {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 ^= u64::from(*b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}


impl<K: core::hash::Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: (0..INITIAL_CAPACITY).map(|_| Vec::new()).collect(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn index(&self, key: &K) -> usize {
        let mut h = MyHasher::new();
        key.hash(&mut h);
        (h.finish() % self.buckets.len() as u64) as usize
    }


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.len >= self.buckets.len() * 3 / 4 {
            self.resize();
        }
        let idx = self.index(&key);
        for (k, v) in self.buckets[idx].iter_mut() {
            if *k == key {
                return Some(core::mem::replace(v, value));
            }
        }
        self.buckets[idx].push((key, value));
        self.len += 1;
        None
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().map(|(k, v)| (k, v)))
    }

    fn resize(&mut self) {
        let mut new_buckets: Vec<Vec<(K, V)>> = 
            (0..self.buckets.len() * 2).map(|_| Vec::new()).collect();
        for bucket in self.buckets.drain(..) {
            for (k, v) in bucket {
                let mut h = MyHasher::new();
                k.hash(&mut h);
                let idx = (h.finish() % new_buckets.len() as u64) as usize;
                new_buckets[idx].push((k, v));
            }
        }
        self.buckets = new_buckets;
    }
}
