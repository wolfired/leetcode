use leetcode::problemset::*;

fn main() {
    println!("{:?}", lcps9::Solution::is_palindrome(0));

    let mut arr = vec![2, 3, 2, 3];
    let d = lcps27::Solution::remove_element(&mut arr, 3);
    println!("{:?} d = {}", arr, d);

    let arr = vec![2, 11, 7, 15];
    let arr = lcps1::Solution::two_sum(arr, 9);
    println!("{:?}", arr);
}
