use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[must_use]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    #[must_use]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mut mid = nums.len() / 2;
        let mut bst = Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::sorted_array_to_bst(nums[..mid].to_owned()),
            right: Self::sorted_array_to_bst(nums[mid + 1..].to_owned()),
        }));

        Some(bst)
    }
}

fn main() {
    let bst = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
    })));
    assert_eq!(bst, Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]));

    let bst = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None,
    })));
    assert_eq!(bst, Solution::sorted_array_to_bst(vec![1, 3]));
}
