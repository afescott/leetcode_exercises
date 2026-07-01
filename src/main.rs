use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    vals: HashMap<String, BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self
            .vals
            .entry(key.clone())
            .and_modify(|val| {
                val.insert(timestamp, value.clone());
            })
            .or_insert(BTreeMap::new());

        let val = entry.iter().find(|(ts, _)| **ts > timestamp);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let res = self.vals.get(&key);

        if let Some(entry) = res {
            let val = entry.range(..=timestamp).next_back();
            if let Some((_, value)) = val {
                return value.clone();
            } else {
                return String::new();
            }
        } else {
            return key;
        }
        /*         } */

        String::new()
        /* let val = entry.iter().find(|(ts, _)| **ts > timestamp);
        let mut returns = String::new();
        if let Some(val) = res {
            let val_2s = self.prev_timestamps.get(&key);
            if let Some(vals) = val_2s {
                if val.1 > vals.1 {
                    returns = val.0.to_string();
                }
            } else {
                returns = val.0.clone();
            }
        } */
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {}
