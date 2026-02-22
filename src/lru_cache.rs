use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::{SystemTime, UNIX_EPOCH},
};

mod merge_sorted_array;
mod reverse_binary;

use std::collections::BTreeMap;

use std::collections::{BTreeMap, HashMap};

struct LRUCacheCorrect {
    capacity: usize,
    counter: u64,
    cache: HashMap<i32, (i32, u64)>, // Key -> (Value, Timestamp)
    times: BTreeMap<u64, i32>,       // Timestamp -> Key
}

impl LRUCacheCorrect {
    fn put(&mut self, key: i32, value: i32) {
        self.counter += 1;

        // 1. If key exists, remove its OLD timestamp from BTreeMap
        if let Some((_, old_ts)) = self.cache.get(&key) {
            self.times.remove(old_ts);
        }

        // 2. Insert/Update the cache and the new timestamp
        self.cache.insert(key, (value, self.counter));
        self.times.insert(self.counter, key);

        // 3. Eviction: If we exceeded capacity
        if self.cache.len() > self.capacity {
            // pop_first() gets the smallest key (oldest timestamp) in O(log N)
            if let Some((old_ts, old_key)) = self.times.pop_first() {
                self.cache.remove(&old_key);
            }
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // 1. Check if the key exists
        if let Some(pair) = self.cache.get_mut(&key) {
            let old_timestamp = pair.1;

            // 2. Increment our global counter to get a fresh "time"
            self.counter += 1;
            let new_timestamp = self.counter;

            // 3. Update the BTreeMap: Remove old time, insert new time
            self.times.remove(&old_timestamp);
            self.times.insert(new_timestamp, key);

            // 4. Update the timestamp stored in the HashMap pair
            pair.1 = new_timestamp;

            // 5. Return the value
            return pair.0;
        }

        -1
    }
}

struct LRUCache {
    capacity: i32,
    counter: u64,
    cache: HashMap<i32, (i32, u64)>, // Key -> (Value, Timestamp)
    times: BTreeMap<u64, i32>,       // Timestamp -> Key
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            counter: 0,
            cache: HashMap::new(),
            times: BTreeMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        println!("Cache: {:?}, Times: {:?}", self.cache, self.times);
        if let Some(val) = self.cache.get_mut(&key) {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            self.times
                .remove_entry(&val.1)
                .and_then(|(_, _)| self.times.insert(time, key));

            val.1 = time;
            return val.0;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        let new_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let least_used_time = self.times.clone();

        let least_used_time = least_used_time.iter().max();

        if self.cache.len() == self.capacity as usize {
            if let Some((least_used_time, least_used_key)) = least_used_time {
                self.cache.entry(key).and_modify(|(old_value, time)| {
                    *old_value = value;
                    *time = new_time;
                });
                self.times.remove_entry(&least_used_time);
                /*                 self.times.remove_entry(least_used_key); */
                self.times.insert(new_time, key);
            }
        } else {
            self.cache.insert(key, (value, new_time));
        }
    }

    fn move_to_head(&mut self, key: usize) {}
}
/* // Initial implementation? Why is it wrong? Well Hashmap is unordered. Don't be afraid to expand
// the signature it does not cause time complexity to go up
struct LRUCache {
    capacity: i32,
    hashset: HashMap<i32, u64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            hashset: HashMap::new(),
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(val) = self.hashset.get_key_value(&key) {
            *val.0
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if self.hashset.len() == self.capacity as usize {
            let val = self.hashset.entry(key);

            if val.key() == &key {
                val.or_insert(time);
            } else {
                let mut val = self.hashset.iter().max_by_key(|f| f.1);
                self.hashset.remove(&val.unwrap().0);
            }
        } else {
            self.hashset.entry(key).or_insert(time);
        }
    }
} */

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

