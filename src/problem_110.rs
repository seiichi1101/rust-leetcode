use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            1 + max(
                Self::height(node.borrow().left.clone()),
                Self::height(node.borrow().right.clone()),
            )
        } else {
            -1
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let left = root.borrow().left.clone();
            let right = root.borrow().right.clone();
            (Self::height(left.clone()) - Self::height(right.clone())).abs() < 2
                && Self::is_balanced(left)
                && Self::is_balanced(right)
        } else {
            true
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_110, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        assert_eq!(problem_110::Solution::is_balanced(input), true);
    }
}
