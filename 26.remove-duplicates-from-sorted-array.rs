/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = nums.len();

        if 2 > k {
            return k as i32;
        }

        k = 1;

        for i in 1..nums.len() {
            let mut j = 0;
            if !loop {
                if k <= j {
                    break false;
                }
                if nums[j] == nums[i] {
                    break true;
                }
                j += 1;
            } {
                nums.swap(k, i);
                k += 1;
            }
        }

        k as i32
    }
}
// @lc code=end

