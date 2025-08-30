pub struct Solution;

impl Solution {
    #[must_use]
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }

        let mut triangle = vec![vec![1]];
        for num_row in 1..row_index {
            triangle.push(Self::create_row(&triangle, num_row as usize));
        }

        Self::create_row(&triangle, row_index as usize)
    }

    fn create_row(triangle: &[Vec<i32>], num_row: usize) -> Vec<i32> {
        let mut row = Vec::with_capacity(num_row);
        row.push(1);
        row.push(1);

        for index in 0..num_row - 1 {
            let prev1 = triangle[num_row - 1][index];
            let prev2 = triangle[num_row - 1][index + 1];
            row.insert(index + 1, prev1 + prev2);
        }

        row
    }
}

fn main() {
    assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
    assert_eq!(vec![1], Solution::get_row(0));
    assert_eq!(vec![1, 1], Solution::get_row(1));
}
