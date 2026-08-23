use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map = HashMap::new();
        // ele = nan, 2nd in ana
        for ele in strs {
            // Bytes for each potential letter in the alphabet
            let mut key = [0u8; 26];
            for bytes in ele.bytes() {
                println!("{:?}", bytes);

                key[(bytes - b'a') as usize] += 1;
                println!("key : {:?}", key[(bytes - b'a') as usize]);
            }
            hash_map.entry(key).or_insert(Vec::new()).push(ele);
            println!("hash_map : {:?}", hash_map);
        }
        hash_map.into_values().collect()
    }
}
