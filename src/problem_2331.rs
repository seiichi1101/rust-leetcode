use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.unwrap();
        let n = node.borrow();
        if n.val < 2 {
            // 0:False, 1:True
            n.val != 0
        } else {
            // 2:OR, 3:AND
            if n.val == 2 {
                Self::evaluate_tree(n.left.clone()) || Self::evaluate_tree(n.right.clone())
            } else {
                Self::evaluate_tree(n.left.clone()) && Self::evaluate_tree(n.right.clone())
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_2331::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(2), Some(1), Some(3), None, None, Some(0), Some(1)],
        );
        let output = true;
        assert_eq!(problem_2331::Solution::evaluate_tree(input), output);
    }
}
