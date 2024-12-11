use std::cell::RefCell;
use std::cmp;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;

impl Solution {
    fn recursive(node: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node_ref) = node {
            let node = node_ref.borrow();
            let left = Self::recursive(&node.left, max);
            let right = Self::recursive(&node.right, max);

            if left + right > *max {
                *max = left + right;
            }
            cmp::max(left, right) + 1
        } else {
            0
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::recursive(&root, &mut max);
        max
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_543, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(problem_543::Solution::diameter_of_binary_tree(input), 3);
    }
    #[test]
    fn test2() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2)]);
        assert_eq!(problem_543::Solution::diameter_of_binary_tree(input), 1);
    }
}
