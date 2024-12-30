use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back((r, None));
        }
        while !queue.is_empty() {
            let mut x_found = None;
            let mut y_found = None;
            for _ in 0..queue.len() {
                if let Some((node, parent_val)) = queue.pop_front() {
                    let n = node.borrow();
                    if n.val == x {
                        x_found = parent_val;
                    }
                    if n.val == y {
                        y_found = parent_val;
                    }
                    if let Some(l) = n.left.clone() {
                        queue.push_back((l, Some(n.val)));
                    }
                    if let Some(r) = n.right.clone() {
                        queue.push_back((r, Some(n.val)));
                    }
                }
            }
            if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
                return x_parent != y_parent;
            }
        }
        false
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_993::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), Some(4)]);
        // let (x, y) = (4, 3);
        // let output = false;
        // assert_eq!(problem_993::Solution::is_cousins(input, x, y), output);

        // let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), None, Some(4)]);
        // let (x, y) = (2, 3);
        // let output = false;
        // assert_eq!(problem_993::Solution::is_cousins(input, x, y), output);

        let input = TreeNode::from_vec(
            0,
            vec![Some(1), Some(2), Some(3), None, Some(4), None, Some(5)],
        );
        let (x, y) = (5, 4);
        let output = true;
        assert_eq!(problem_993::Solution::is_cousins(input, x, y), output);
    }
}
