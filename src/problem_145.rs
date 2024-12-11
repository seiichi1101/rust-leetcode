use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(head) = root {
            let left = Self::postorder_traversal(head.borrow().left.clone());
            let right = Self::postorder_traversal(head.borrow().right.clone());
            res.extend(left);
            res.extend(right);
            res.push(head.borrow().val);
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_145, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), None, Some(2), None, None, Some(3)]);
        let output = vec![3, 2, 1];
        assert_eq!(problem_145::Solution::postorder_traversal(input), output);
    }
}
