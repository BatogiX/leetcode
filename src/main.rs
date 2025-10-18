struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let max_candies = candy_type.len() / 2;
        let unique_candy_types = candy_type.into_iter().collect::<HashSet<_>>();
        max_candies.min(unique_candy_types.len()) as i32
    }
}

fn main() {
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
    assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
}
