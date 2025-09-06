struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] = -nums[index];
            }
        }

        let mut disappeared_numbers = vec![];
        for (index, &num) in nums.iter().enumerate() {
            if num > 0 {
                disappeared_numbers.push((index + 1) as i32);
            }
        }

        disappeared_numbers
    }
}

fn main() {
    assert_eq!(
        vec![5, 6],
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    assert_eq!(
        vec![5, 6, 9],
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 8, 2, 3, 1])
    );
    assert_eq!(
        vec![5, 6, 9, 10],
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 8, 8, 2, 3, 1])
    );
    assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    assert_eq!(vec![1], Solution::find_disappeared_numbers(vec![2, 2]));
    assert_eq!(
        Vec::<i32>::new(),
        Solution::find_disappeared_numbers(vec![1, 2])
    );
    assert_eq!(
        vec![1, 3],
        Solution::find_disappeared_numbers(vec![2, 2, 2])
    );
}
