use std::cell::RefCell;
use std::cmp::min;
use std::i32;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MAX;
        let mut stack = vec![];
        let mut prev = i32::MAX;
        let mut current = root;
        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }

            current = stack.pop();
            if let Some(node) = current {
                let val = node.borrow().val;
                res = min(res, (val - prev).abs());
                prev = val;
                current = node.borrow().right.clone();
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_530::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
        let output = 1;
        assert_eq!(problem_530::Solution::get_minimum_difference(input), output);
    }
}
