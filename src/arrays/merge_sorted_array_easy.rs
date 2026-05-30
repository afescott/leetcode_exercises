//D+ GRADE
/* impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if !nums1.is_empty() {
            for i in 0..nums2.len() {
                if nums2.len() == 0 || nums1.len() == 1 {
                    nums1[n as usize - 1] = nums2[i];
                } else {
                    nums1[i + n as usize] = nums2[i];
                }

                println!("nums 1: {:?}", nums1);
            }
            nums1.sort_by(|a, b| a.cmp(b));
        }
    }
} */
