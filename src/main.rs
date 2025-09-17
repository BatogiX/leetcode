struct Solution;

impl Solution {
    pub fn matrix_reshape(mut mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let n = (r * c) as usize;
        
        if n != mat.len() * mat[0].len() {
            return mat;
        }

        let flat_mat = if mat.len() == 1 {
            mat.pop().unwrap()
        } else {
            let mut flat_mat = Vec::with_capacity(n);

            for row in mat {
                for column in row {
                    flat_mat.push(column);
                }
            }

            flat_mat
        };

        let mut reshaped_mat = Vec::with_capacity(n);
        for chunk in flat_mat.chunks(c as usize) {
            reshaped_mat.push(Vec::from(chunk));
        }

        reshaped_mat
    }
}

fn main() {
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );

    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );

    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2, 3, 4]], 2, 2),
        vec![vec![1, 2], vec![3, 4]]
    );

    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2, 3, 4, 5, 6, 7, 8]], 4, 2),
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]
    );

    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2]], 1, 1),
        vec![vec![1, 2]]
    );
}
