fn main() {
    assert_eq!(2, search_insert(vec![1, 3, 5, 6], 5));
    assert_eq!(1, search_insert(vec![1, 3, 5, 6], 2));
    assert_eq!(4, search_insert(vec![1, 3, 5, 6], 7));
    assert_eq!(6, search_insert(vec![1, 3, 5, 6, 8, 11, 123, 123], 100));
    assert_eq!(0, search_insert(vec![1, 3, 5, 6], 0));
    assert_eq!(8, search_insert(vec![1, 3, 5, 6, 8, 11, 123, 123], 1000));
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = (left + right) / 2;

        if target == nums[mid] {
            return mid as i32;
        }

        if target < nums[mid] {
            right = mid
        } else {
            left = mid + 1
        }
    }

    left as i32
}
