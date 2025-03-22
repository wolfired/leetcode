/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut c = 1;
        let mut r = digits
            .iter_mut()
            .rev()
            .map(|d| {
                let tmp = *d + c;
                c = tmp / 10;
                tmp % 10
            })
            .collect::<Vec<_>>();
        r.reverse();
        if 0 < c {
            r.insert(0, c);
        }
        r
    }
}
// @lc code=end

