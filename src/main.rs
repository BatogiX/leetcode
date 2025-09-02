pub struct Solution;

impl Solution {
    #[must_use]
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let len = nums.len();
        let mut ranges = vec![];
        let mut memorized_num = None;

        if len == 0 {
            return ranges;
        }

        if len == 1 {
            ranges.push(nums[0].to_string());
            return ranges;
        }

        for i in 0..len - 1 {
            if nums[i + 1] - nums[i] == 1 {
                if memorized_num.is_none() {
                    memorized_num = Some(nums[i]);
                }
                continue;
            }

            match memorized_num {
                Some(num) => {
                    ranges.push(format!("{num}->{}", nums[i]));
                    memorized_num = None;
                }
                None => ranges.push(nums[i].to_string()),
            }
        }

        match memorized_num {
            Some(num) => ranges.push(format!("{num}->{}", nums[len - 1])),
            None => ranges.push(nums[len - 1].to_string()),
        }

        ranges
    }
}

fn main() {
    assert_eq!(
        vec!["0->2", "4->5", "7"],
        Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
    );

    assert_eq!(
        vec!["0", "2->4", "6", "8->9"],
        Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
    );

    assert_eq!(
        vec!["0->2", "4->6"],
        Solution::summary_ranges(vec![0, 1, 2, 4, 5, 6])
    );

    assert_eq!(Vec::<String>::new(), Solution::summary_ranges(vec![]));
    assert_eq!(vec!["1"], Solution::summary_ranges(vec![1]));
    assert_eq!(vec!["1->2"], Solution::summary_ranges(vec![1, 2]));
}
