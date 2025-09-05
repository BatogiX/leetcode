struct Solution;

impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 {
            return nums[0];
        }

        if len == 2 {
            return nums[0].max(nums[1]);
        }

        nums.sort_unstable();
        let mut nums_iter_rev = nums.iter().rev();
        let mut prev = nums_iter_rev.next().unwrap();
        let mut count = 1;

        for num in nums_iter_rev {
            if num != prev {
                if count == 2 {
                    return *num;
                }
                count += 1;
                prev = num;
            }
        }

        nums[len - 1]
    }
}

fn main() {
    assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
    assert_eq!(2, Solution::third_max(vec![1, 2]));
    assert_eq!(1, Solution::third_max(vec![2, 2, 3, 1]));
    assert_eq!(2, Solution::third_max(vec![1, 1, 2]));
}
