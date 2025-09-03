pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                j += 1;
            }
        }

        for i in j..nums.len() {
            nums[i] = 0;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(vec![1, 3, 12, 0, 0], nums);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(vec![0], nums);

    let mut nums = vec![0, 0, 1];
    Solution::move_zeroes(&mut nums);
    assert_eq!(vec![1, 0, 0], nums);
}
