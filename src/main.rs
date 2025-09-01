pub struct Solution;

use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    #[must_use]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hm = HashMap::with_capacity(nums.len());

        for (index, num) in nums.iter().enumerate() {
            match hm.entry(num) {
                Entry::Occupied(mut occupied_entry) => {
                    if index.abs_diff(*occupied_entry.get()) <= k as usize {
                        return true;
                    }

                    occupied_entry.insert(index);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(index);
                }
            }
        }

        false
    }
}

fn main() {
    assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    assert!(!Solution::contains_nearby_duplicate(
        vec![1, 2, 3, 1, 2, 3],
        2
    ));
}
