struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut fed_children = 0;
        g.sort_unstable();
        s.sort_unstable();
        s.reverse();

        let mut index = 0;
        while index < g.len() {
            let Some(content) = s.pop() else { break };

            if content >= g[index] {
                fed_children += 1;
                index += 1;
            }
        }

        fed_children
    }
}

fn main() {
    assert_eq!(
        1,
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
    );

    assert_eq!(
        2,
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
    );

    assert_eq!(0, Solution::find_content_children(vec![1], vec![]));
    assert_eq!(1, Solution::find_content_children(vec![1], vec![1, 2]));
    assert_eq!(0, Solution::find_content_children(vec![2, 2], vec![1, 1]));
    assert_eq!(0, Solution::find_content_children(vec![2, 2], vec![1]));
    assert_eq!(
        2,
        Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8])
    );
}
