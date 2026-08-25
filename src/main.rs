// Input: nums = [1,1,2]
// Output: [[1,1,2],[1,2,1],[2,1,1]]

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // duplicates must sit together

        let mut solution = Vec::new();
        let mut current_rota = Vec::new();
        let mut used = vec![false; nums.len()];

        back_track(&nums, &mut current_rota, &mut used, &mut solution);
        solution
    }
}

fn back_track(
    nums: &[i32],
    current_rota: &mut Vec<i32>,
    used: &mut [bool],
    perms: &mut Vec<Vec<i32>>,
) {
    if nums.len() == current_rota.len() {
        perms.push(current_rota.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        // skip duplicate at this level (only use leftmost copy first)
        if i > 0 {
            let same_as_previous = nums[i] == nums[i - 1];

            let previous_not_used = !used[i - 1];

            if same_as_previous && previous_not_used {
                continue;
            }
        }

        used[i] = true;
        current_rota.push(nums[i]);
        back_track(nums, current_rota, used, perms);
        current_rota.pop();
        used[i] = false;
    }
}

fn main() {}
