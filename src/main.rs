use std::collections::{HashMap, hash_map::Entry};

struct Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let min_len = nums1.len().min(nums2.len());
        let mut hm = HashMap::with_capacity(min_len);
        let mut answer = Vec::with_capacity(min_len);

        if min_len == nums1.len() {
            (nums1, nums2) = (nums2, nums1);
        }

        for num in nums1 {
            match hm.entry(num) {
                Entry::Occupied(mut occupied_entry) => *occupied_entry.get_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        }

        for num in nums2 {
            if let Entry::Occupied(mut occupied_entry) = hm.entry(num) {
                let value = occupied_entry.get_mut();

                if *value == 1 {
                    occupied_entry.remove();
                } else {
                    *value -= 1;
                }

                answer.push(num);
            }
        }

        answer
    }
}

fn main() {
    assert_eq!(
        vec![2, 2],
        Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
    );

    assert_eq!(
        vec![4, 9],
        Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
    );

    assert_eq!(vec![1], Solution::intersect(vec![3, 1, 2], vec![1, 1]));
}
