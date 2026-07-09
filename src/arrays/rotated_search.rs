impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for (counter, ele) in nums.iter().enumerate() {
            if *ele == target {
                return counter as i32;
            }
        }
        -1
    }
}
