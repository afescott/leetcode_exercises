impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].clone();

        for s in &strs[1..] {
            while !s.starts_with(&prefix) {
                prefix.pop();
            }
        }

        prefix

        /* //prev solution unnecessary hashmap and 0(log n)2
        if strs.len() == 1 {
            return strs.first().unwrap().to_string();
        }
        let mut prefix = String::new();

        let mut hash_map: HashMap<usize, String> = HashMap::new();

        let shortest = strs.iter().min_by_key(|s| s.len()).unwrap();

        for i in 0..shortest.len() {
            for ele in &strs {
                let val = &ele[i..i + 1];

                let hash_val = hash_map.entry(i).or_insert(val.to_string());

                if hash_val != val {
                    prefix.pop().unwrap();

                    return prefix;
                }

                if prefix.len() <= i {
                    prefix.push_str(val);
                }
            }
        }
        prefix */
    }
}
