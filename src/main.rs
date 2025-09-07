struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_consecutive_ones = 0;
        let mut consecutive_ones = 0;

        for num in nums {
            if num == 1 {
                consecutive_ones += 1;
            } else {
                if consecutive_ones > max_consecutive_ones {
                    max_consecutive_ones = consecutive_ones;
                }

                consecutive_ones = 0;
            }
        }

        if consecutive_ones > max_consecutive_ones {
            max_consecutive_ones = consecutive_ones;
        }

        max_consecutive_ones
    }
}

fn main() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );

    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
        2
    );

    assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
    assert_eq!(Solution::find_max_consecutive_ones(vec![0]), 0);
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
        2
    );
}
