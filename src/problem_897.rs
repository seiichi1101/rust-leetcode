use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = None;
        let mut current = root.clone();
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        let mut stack = vec![];
        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let mut n = node.borrow_mut();
                if head.is_none() {
                    head = Some(node.clone());
                }
                if let Some(p) = prev {
                    p.borrow_mut().right = Some(node.clone());
                }
                n.left = None;
                prev = Some(node.clone());
                current = n.right.clone();
            }
        }
        println!("{:?}", head);
        head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_897::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(4), None, None, Some(3)]);
        let output = TreeNode::from_vec(
            0,
            vec![
                Some(1),
                None,
                Some(2),
                None,
                None,
                None,
                Some(3),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
            ],
        );
        assert_eq!(problem_897::Solution::increasing_bst(input), output);
    }
}
