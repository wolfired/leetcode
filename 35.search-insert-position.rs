/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        fn work(nums: &[i32], target: i32) -> usize {
            let mid = nums.len() / 2;
            if target < nums[mid] {
                if 0 < mid {
                    work(&nums[0..mid], target)
                } else {
                    mid
                }
            } else if nums[mid] < target {
                if 0 < mid {
                    mid + work(&nums[mid..], target)
                } else {
                    mid + 1
                }
            } else {
                mid
            }
        };
        work(&nums, target) as i32
    }
}
// @lc code=end

