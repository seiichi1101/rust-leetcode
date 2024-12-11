use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(head) = root {
            res.push(head.borrow().val);
            let left = Self::preorder_traversal(head.borrow().left.clone());
            let right = Self::preorder_traversal(head.borrow().right.clone());
            res.extend(left);
            res.extend(right);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_144, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), None, Some(2), None, None, Some(3)]);
        let output = vec![1, 2, 3];
        assert_eq!(problem_144::Solution::preorder_traversal(input), output);
    }
}
