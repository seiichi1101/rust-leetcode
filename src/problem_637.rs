use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::TreeNode;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let len = queue.len();
            let mut sum = 0f64;
            for _ in 0..len {
                let node = queue.pop_front().unwrap().unwrap();
                let n = node.borrow();
                sum += n.val as f64;
                if n.left.is_some() {
                    queue.push_back(n.left.clone());
                }
                if n.right.is_some() {
                    queue.push_back(n.right.clone());
                }
            }
            res.push(sum / len as f64);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_637::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        // );
        // let output = vec![3.00000, 14.50000, 11.00000];
        let input = TreeNode::from_vec(
            0,
            vec![Some(2147483647), Some(2147483647), Some(2147483647)],
        );
        let output = vec![2147483647.00000, 2147483647.00000];
        assert_eq!(problem_637::Solution::average_of_levels(input), output);
    }
}
