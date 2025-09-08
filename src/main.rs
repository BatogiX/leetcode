struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let nums2_hm = {
            let mut nums2_hm = HashMap::with_capacity(nums2.len());

            for (index, num) in nums2.iter().enumerate() {
                nums2_hm.insert(num, index);
            }

            nums2_hm
        };

        for num1 in nums1 {
            let nums2_index = nums2_hm.get(&num1).unwrap();
            for i in nums2_index + 1..=nums2.len() {
                match nums2.get(i) {
                    Some(&num2) => {
                        if num2 > num1 {
                            ans.push(num2);
                            break;
                        }
                    }
                    None => ans.push(-1),
                }
            }
        }

        ans
    }
}

fn main() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );

    assert_eq!(
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
