pub struct Solution;

use std::collections::HashSet;

impl Solution {
    #[must_use]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hs = HashSet::with_capacity(nums.len());
        !nums.iter().all(|&num| hs.insert(num))
    }
}

fn main() {
    assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    assert!(Solution::contains_duplicate(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}
