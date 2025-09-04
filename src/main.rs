use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let min_len = nums1.len().min(nums2.len());
        let mut unique = HashSet::with_capacity(min_len);
        let mut answer = Vec::with_capacity(min_len);

        if min_len != nums1.len() {
            (nums1, nums2) = (nums2, nums1);
        }

        for num in nums1 {
            unique.insert(num);
        }

        for num in nums2 {
            if unique.is_empty() {
                break;
            }

            if unique.contains(&num) {
                answer.push(num);
                unique.remove(&num);
            }
        }

        answer
    }
}

fn main() {
    assert_eq!(
        vec![2],
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])
    );

    assert_eq!(
        vec![9, 4],
        Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
    );
}
