pub struct Solution;

impl Solution {
    #[must_use]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let majority = nums.len() / 2;

        let mut count = 1;
        let mut prev = nums[0];
        for next in &nums[1..] {
            if *next == prev {
                count += 1;
            } else {
                prev = *next;
                count = 1;
                continue;
            }

            if count > majority {
                return *next;
            }
        }

        prev
    }
}

fn main() {
    assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    assert_eq!(1, Solution::majority_element(vec![1]));
}
