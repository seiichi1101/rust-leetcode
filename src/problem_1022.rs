use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn sum(root: Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let val = n.val + num * 2;
            if n.left.is_none() && n.right.is_none() {
                val
            } else {
                Self::sum(n.left.clone(), val) + Self::sum(n.right.clone(), val)
            }
        } else {
            0
        }
    }
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum(root, 0)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_1022::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(1),
            ],
        );
        let output = 22;
        assert_eq!(problem_1022::Solution::sum_root_to_leaf(input), output);
    }
}
