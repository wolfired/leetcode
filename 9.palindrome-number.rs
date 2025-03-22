/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if 0 > x { return false; }
        if 10 > x { return true; }

        let mut y = 0;

        {
            let mut x = x;
            while 0 < x {
                y = y * 10 + (x % 10);
                x /= 10;
            }
        }

        return x == y;
    }
}
// @lc code=end
