use std::cell::RefCell;
use std::cmp::min;
use std::i32;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        let mut current = root;
        let mut prev = i32::MAX;
        let mut min_diff = i32::MAX;

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }

            current = stack.pop();
            if let Some(node) = current {
                let val = node.borrow().val;
                min_diff = min(min_diff, (val - prev).abs());
                prev = val;
                current = node.borrow().right.clone();
            }
        }
        min_diff
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use crate::{
        problem_783::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(0, vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
        // let output = 1;
        // let input = TreeNode::from_vec(0, vec![Some(90), Some(69), None, Some(49), Some(89)]);
        // let output = 1;
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(96),
                Some(12),
                None,
                None,
                Some(13),
                None,
                None,
                None,
                None,
                None,
                Some(52),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(29),
            ],
        );
        let output = 1;
        assert_eq!(problem_783::Solution::min_diff_in_bst(input), output);
    }
}
