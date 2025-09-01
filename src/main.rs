pub struct Solution;

impl Solution {
    #[must_use]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut single_num = nums[0];

        for num in &nums[1..] {
            single_num ^= num;
        }

        single_num
    }
}

fn main() {
    assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    assert_eq!(1, Solution::single_number(vec![1]));
}
