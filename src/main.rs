pub struct Solution;

impl Solution {
    #[must_use]
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        for (index, num) in nums.iter().enumerate() {
            if *num != index as i32 {
                return index as i32;
            }
        }

        nums.len() as i32
    }
}

fn main() {
    assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
    assert_eq!(2, Solution::missing_number(vec![0, 1]));
    assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    assert_eq!(0, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 8, 1]));
    assert_eq!(0, Solution::missing_number(vec![1]));
    assert_eq!(1, Solution::missing_number(vec![0]));
    assert_eq!(0, Solution::missing_number(vec![2]));
}
