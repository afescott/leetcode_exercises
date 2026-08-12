use std::{
    collections::{hash_set, HashMap, HashSet, VecDeque},
    vec,
};

struct RandomizedSet {
    values: Vec<i32>,
    hash_map: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let check = self.hash_map.get(&val);

        if check.is_some() {
            return false;
        }
        let len = self.values.len();
        self.hash_map.insert(val, len);
        self.values.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        let Some(&idx) = self.hash_map.get(&val) else {
            return false;
        };

        // gets last
        let last = *self.values.last().unwrap();

        println!("index {:?}", self.values[idx]);
        self.values[idx] = last;

        self.hash_map.insert(last, idx);

        self.values.pop();
        self.hash_map.remove(&val);

        true
    }

    fn get_random(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.values.len());
        self.values[i]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

