use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn flatten_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(head) = root {
            let left = head.borrow().left.clone();
            let right = head.borrow().right.clone();
            if left.is_none() && right.is_none() {
                return Some(head.clone());
            }

            let left_tail = Self::flatten_tree(&mut left.clone());
            let right_tail = Self::flatten_tree(&mut right.clone());

            if left_tail.is_some() {
                left_tail.clone().unwrap().borrow_mut().right = right.clone();
                head.borrow_mut().right = left.clone();
                head.borrow_mut().left = None;
            }

            if right_tail.is_some() {
                right_tail
            } else {
                left_tail
            }
        } else {
            None
        }
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::flatten_tree(root);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_114, TreeNode};

    #[test]
    fn test1() {
        let mut input = TreeNode::from_vec(
            0,
            vec![Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)],
        );
        let output = TreeNode::from_vec(
            0,
            vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
                None,
                Some(6),
            ],
        );
        problem_114::Solution::flatten(&mut input);
        assert_eq!(input, output);
    }
}
