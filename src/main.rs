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
    let mut right = nums.len() as i32 - 1;
    let mut answer = nums.len() as i32;

    while left <= right {
        let mid = left + (right - left) / 2;

        if target == nums[mid as usize] {
            return mid;
        }

        if target < nums[mid as usize] {
            right = mid - 1;
            answer = mid;
        } else {
            left = mid + 1
        }
    }

    answer
}
