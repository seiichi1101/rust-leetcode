use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut first = -1;
        let mut second = -1;
        let mut queue = VecDeque::new();

        queue.push_back(root);

        while let Some(Some(node)) = queue.pop_front() {
            let n = node.borrow();
            if first == -1 {
                first = n.val;
            } else if second == -1 && n.val > first {
                second = n.val;
            } else if first < n.val && n.val < second {
                second = n.val;
            }
            queue.push_back(n.left.clone());
            queue.push_back(n.right.clone());
        }

        second
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_671::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)],
        // );
        // let output = 5;
        // let input = TreeNode::from_vec(0, vec![Some(2), Some(2), Some(2)]);
        // let output = -1;
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(1),
                Some(1),
                Some(1),
                Some(1),
                Some(5),
                Some(1),
                Some(4),
            ],
        );
        let output = 4;
        assert_eq!(
            problem_671::Solution::find_second_minimum_value(input),
            output
        );
    }
}
