use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.unwrap();
        let n = node.borrow();
        n.val == (n.left.clone().unwrap().borrow().val + n.right.clone().unwrap().borrow().val)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_2236::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(10), Some(4), Some(6)]);
        let output = true;
        assert_eq!(problem_2236::Solution::check_tree(input), output);
    }
}
