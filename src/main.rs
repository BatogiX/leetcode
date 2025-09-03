struct NumArray {
    nums: Vec<i32>,
    pre_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut pre_sum: Vec<i32> = vec![0; nums.len()];
        pre_sum[0] = nums[0];

        for i in 1..nums.len() {
            pre_sum[i] = pre_sum[i - 1] + nums[i];
        }

        Self { nums, pre_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return self.pre_sum[right as usize] - self.pre_sum[left as usize] + self.nums[left as usize];
    }
}

fn main() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(1, obj.sum_range(0, 2));
    assert_eq!(-1, obj.sum_range(2, 5));
    assert_eq!(-3, obj.sum_range(0, 5));
}
