use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            true
        } else if let Some(node) = root {
            let node = node.borrow();
            Self::is_subtree(node.left.clone(), sub_root.clone())
                || Self::is_subtree(node.right.clone(), sub_root.clone())
        } else {
            false
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_572::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input1 = TreeNode::from_vec(0, vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let input2 = TreeNode::from_vec(0, vec![Some(4), Some(1), Some(2)]);
        assert_eq!(problem_572::Solution::is_subtree(input1, input2), true);
    }
}
