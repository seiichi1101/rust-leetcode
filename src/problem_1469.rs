use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn get_lonely_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        stack.push(root);
        while !stack.is_empty() {
            if let Some(Some(node)) = stack.pop() {
                let n = node.borrow();
                match (n.left.clone(), n.right.clone()) {
                    (None, None) => (),
                    (None, Some(r)) => {
                        res.push(r.borrow().val);
                        stack.push(Some(r));
                    }
                    (Some(l), None) => {
                        res.push(l.borrow().val);
                        stack.push(Some(l));
                    }
                    (Some(l), Some(r)) => {
                        stack.push(Some(l));
                        stack.push(Some(r));
                    }
                }
            }
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_1469::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), None, Some(4)]);
        let output = vec![4];
        assert_eq!(problem_1469::Solution::get_lonely_nodes(input), output);
    }
}
