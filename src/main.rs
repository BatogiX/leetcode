struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;

        if r * c != mat.len() * mat[0].len() {
            return mat;
        }

        let flat_mat: Vec<i32> = mat.into_iter().flatten().collect();

        let mut reshaped_mat = Vec::with_capacity(r * c);
        for chunk in flat_mat.chunks(c) {
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
