fn main() {
    assert_eq!(2, search_insert(vec![1, 3, 5, 6], 5));
    assert_eq!(1, search_insert(vec![1, 3, 5, 6], 2));
    assert_eq!(4, search_insert(vec![1, 3, 5, 6], 7));
    assert_eq!(4, search_insert(vec![1, 3, 5, 6, 8], 7));
    assert_eq!(6, search_insert(vec![1, 3, 5, 6, 8, 11, 123, 123], 100));
    assert_eq!(0, search_insert(vec![1, 3, 5, 6], 0));
    assert_eq!(8, search_insert(vec![1, 3, 5, 6, 8, 11, 123, 123], 1000));
    assert_eq!(6, search_insert(vec![1, 3, 5, 6, 8, 11, 123, 124], 123));
}

#[must_use]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;

    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Less => right = mid - 1,
            Ordering::Equal => return mid,
            Ordering::Greater => left = mid + 1,
        }
    }

    if right < 0 {
        right = 0;
    }

    match target.cmp(&nums[right as usize]) {
        Ordering::Less => match right {
            0 => right,
            _ => right - 1,
        },
        Ordering::Equal => right,
        Ordering::Greater => right + 1,
    }
}
