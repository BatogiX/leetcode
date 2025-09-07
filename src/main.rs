struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if duration == 0 {
            return 0;
        }

        let mut total_poisoned_duration = 0;
        for index in 0..time_series.len() - 1 {
            total_poisoned_duration += (time_series[index + 1] - time_series[index]).min(duration);
        }

        total_poisoned_duration += duration;
        total_poisoned_duration
    }
}

fn main() {
    assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    assert_eq!(Solution::find_poisoned_duration(vec![1, 3], 3), 5);
    assert_eq!(Solution::find_poisoned_duration(vec![1, 4, 6], 2), 6);
}
