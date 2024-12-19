use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut val = None;
        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(Some(node)) = stack.pop() {
                let n = node.borrow();
                if val.is_none() {
                    val = Some(n.val);
                } else if val.unwrap() != n.val {
                    return false;
                }
                stack.push(n.left.clone());
                stack.push(n.right.clone());
            }
        }
        true
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_965::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(1), Some(1), Some(1), Some(1), Some(1), None, Some(1)],
        // );
        // let output = true;
        let input = TreeNode::from_vec(0, vec![Some(2), Some(2), Some(2), Some(5), Some(2)]);
        let output = false;
        assert_eq!(problem_965::Solution::is_unival_tree(input), output);
    }
}
