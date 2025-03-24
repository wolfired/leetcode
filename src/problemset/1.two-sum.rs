/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut objs = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
        objs.sort_by_key(|o| o.1);
        for o in objs.iter().enumerate() {
            if let Ok(i) = objs[{ o.0 + 1 }..].binary_search_by(|n| n.1.cmp(&(target - { o.1 .1 })))
            {
                return vec![o.1 .0 as i32, objs[{ o.0 + 1 }..][i].0 as i32];
            }
        }
        return vec![];
    }
}
// @lc code=end
