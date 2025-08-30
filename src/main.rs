pub struct Solution;

impl Solution {
    #[must_use]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = vec![vec![1]];

        if num_rows == 1 {
            return triangle;
        }

        for num_row in 2..=num_rows {
            let mut row = Vec::with_capacity(num_row as usize);
            row.push(1);
            row.push(1);

            for index in 1..num_row - 1 {
                let prev1 = triangle[num_row as usize - 2][index as usize - 1];
                let prev2 = triangle[num_row as usize - 2][index as usize];
                row.insert(index as usize, prev1 + prev2);
            }

            triangle.push(row);
        }

        triangle
    }
}

fn main() {
    assert_eq!(
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ],
        Solution::generate(5)
    );

    assert_eq!(vec![vec![1]], Solution::generate(1));
}
