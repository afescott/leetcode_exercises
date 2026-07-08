impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut best_sum = nums[0] + nums[1] + nums[2];
        let mut best_diff = (best_sum - target).abs();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let sum = nums[i] + nums[j] + nums[k];
                    let diff = (sum - target).abs();

                    if diff < best_diff {
                        best_diff = diff;
                        best_sum = sum;
                    }
                }
            }
        }
        best_sum
    }
}
