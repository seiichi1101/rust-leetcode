use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let n = node.borrow();
            if val == n.val {
                root
            } else if val < n.val {
                Self::search_bst(n.left.clone(), val)
            } else {
                Self::search_bst(n.right.clone(), val)
            }
        } else {
            None
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_700::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input1 = TreeNode::from_vec(0, vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
        let input2 = 2;
        let output = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]);
        assert_eq!(problem_700::Solution::search_bst(input1, input2), output);
    }
}
