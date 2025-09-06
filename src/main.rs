struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let rows_len = grid.len() as i32;
        let columns_len = grid[0].len() as i32;
        let mut total_perimeter = 0;

        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == &1 {
                    for cord in [-1, 1] {
                        let mut adj = x as i32 + cord;
                        if adj == -1 || adj == columns_len || grid[y][adj as usize] == 0 {
                            total_perimeter += 1;
                        }

                        adj = y as i32 + cord;
                        if adj == -1 || adj == rows_len || grid[adj as usize][x] == 0 {
                            total_perimeter += 1;
                        }
                    }
                }
            }
        }

        total_perimeter
    }
}

fn main() {
    assert_eq!(
        16,
        Solution::island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ])
    );

    assert_eq!(4, Solution::island_perimeter(vec![vec![1]]));
    assert_eq!(4, Solution::island_perimeter(vec![vec![1, 0]]));
}
