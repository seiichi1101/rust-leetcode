use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        let mut res = 0;
        while let Some(node) = queue.pop_front() {
            let val = node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            match (left, right) {
                (None, None) => {
                    res += val;
                }
                (Some(l), None) => {
                    l.borrow_mut().val += val * 10;
                    queue.push_back(l);
                }
                (None, Some(r)) => {
                    r.borrow_mut().val += val * 10;
                    queue.push_back(r);
                }
                (Some(l), Some(r)) => {
                    l.borrow_mut().val += val * 10;
                    queue.push_back(l);
                    r.borrow_mut().val += val * 10;
                    queue.push_back(r);
                }
            }
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_129, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3)]);
        let output = 25;
        assert_eq!(problem_129::Solution::sum_numbers(input), output);
    }
}
