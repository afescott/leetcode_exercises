use core::num;
use std::fmt::write;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        for i in 0..nums.len() {
            //Using rev() here would cause both loops to compare with same type
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec
    }
}

fn main() {
    let val = Solution::two_sum(vec![2, 5, 5, 11], 10);

    println!("{:?}", val);
    assert_eq!(val, vec![1, 2]);

