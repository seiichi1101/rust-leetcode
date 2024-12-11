use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        let mut current_level = 0;
        queue.push_back(root);
        while !queue.is_empty() {
            let mut tmp_res = vec![];
            for i in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    tmp_res.push(node.borrow().val);
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                }
            }
            if !tmp_res.is_empty() {
                res.push(tmp_res);
            }
        }
        res.reverse();
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_107, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        let output = vec![vec![15, 7], vec![9, 20], vec![3]];
        assert_eq!(problem_107::Solution::level_order_bottom(input), output);
    }
}
