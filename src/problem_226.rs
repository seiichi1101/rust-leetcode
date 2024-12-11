use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = &root {
            let mut root = root.borrow_mut();
            let left = Self::invert_tree(root.left.take());
            let right = Self::invert_tree(root.right.take());
            root.left = right;
            root.right = left;
        }
        root
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_226::{self, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]);
        let output = TreeNode::from_vec(0, vec![Some(2), Some(3), Some(1)]);
        assert_eq!(problem_226::Solution::invert_tree(input), output);
    }
}
