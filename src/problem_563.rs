use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            node.borrow().val
                + Self::sum(node.borrow().left.clone())
                + Self::sum(node.borrow().right.clone())
        } else {
            0
        }
    }
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            if let Some(next) = queue.pop_front() {
                if let Some(node) = next {
                    sum += (Self::sum(node.borrow().left.clone())
                        - Self::sum(node.borrow().right.clone()))
                    .abs();
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                }
            }
        }
        sum
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_563::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3)]);
        // let output = 1;
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(4), Some(2), Some(9), Some(3), Some(5), None, Some(7)],
        // );
        // let output = 15;
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(21),
                Some(7),
                Some(14),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3),
            ],
        );
        let output = 9;
        assert_eq!(problem_563::Solution::find_tilt(input), output);
    }
}
