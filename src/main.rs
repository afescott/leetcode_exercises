use std::{collections::VecDeque, vec};

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut vec_deque = VecDeque::new();
        let mut result = Vec::new();
        let k = k as usize;

        for i in 0..=nums.len() - k {
            let vals = &nums[i..i + k];
            if i == 0 {
                vec_deque = VecDeque::from(vals.to_vec());
            } else {
                vec_deque.pop_front();
                vec_deque.pop_back();

                vec_deque.push_front(vals[i]);

                vec_deque.push_back(vals[k - 1]);
            }
            let max = vec_deque.iter().max();
            if let Some(max) = max {
                result.push(max.clone());
            }
            println!("{:?}", vec_deque);
        }
        result
    }
}

fn main() {}
