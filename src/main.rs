fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);

    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    merge(&mut nums1, 1, &mut nums2, 0);
    assert_eq!(vec![1], nums1);

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(vec![1], nums1);

    let mut nums1 = vec![2, 0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 1, &mut nums2, 1);
    assert_eq!(vec![1, 2], nums1);

    let mut nums1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
    let mut nums2 = vec![1, 2, 2];
    merge(&mut nums1, 6, &mut nums2, 3);
    assert_eq!(vec![-1, 0, 0, 1, 2, 2, 3, 3, 3], nums1);

    let mut nums1 = vec![-1, -1, 0, 0, 0, 0];
    let mut nums2 = vec![-1, 0];
    merge(&mut nums1, 4, &mut nums2, 2);
    assert_eq!(vec![-1, -1, -1, 0, 0, 0], nums1);
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // Removing trailing zeros
    for _ in 0..nums1.len() - m as usize {
        nums1.remove(nums1.len() - 1);
    }

    if nums1.is_empty() {
        for num2 in nums2.drain(..) {
            nums1.push(num2);
        }
        return;
    }

    let mut nums1_index = m - 1;
    let mut nums2_index = n - 1;

    // Merging
    while nums1_index >= 0 && nums2_index >= 0 {
        let num1 = nums1[nums1_index as usize];
        let num2 = nums2[nums2_index as usize];

        if num2 >= num1 {
            nums2.remove(nums2_index as usize);
            nums1.insert(nums1_index as usize + 1, num2);
            nums2_index -= 1;
        } else {
            nums1_index -= 1;
        }
    }

    if !nums2.is_empty() {
        for num2 in nums2.drain(..).rev() {
            nums1.insert(0, num2);
        }
    }
}
