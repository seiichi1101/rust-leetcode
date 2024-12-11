use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(r) = root {
            let left = Self::inorder_traversal(r.borrow().left.clone());
            let right = Self::inorder_traversal(r.borrow().right.clone());
            res.extend(left);
            res.extend(vec![r.borrow().val]);
            res.extend(right);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_94::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), None, Some(2), None, None, Some(3)]);
        println!("input: {:?}", input);
        assert_eq!(
            problem_94::Solution::inorder_traversal(input),
            vec![1, 3, 2]
        );
    }
}
