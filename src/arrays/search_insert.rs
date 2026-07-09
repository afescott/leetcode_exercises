impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut found = true;
        let mut found_upper = false;
        for (counter, ele) in nums.iter().enumerate() {
            if *ele == target {
                return counter as i32;
            }

            if *ele < target && !found {
                found = true;
            }
            if *ele > target && found && !found_upper {
                /*                 found_upper = true; */
                return counter as i32;
            }
        }
        nums.len() as i32
    }
}
