use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(head) = root {
            queue.push_back(head);
            let mut count = 0;
            while !queue.is_empty() {
                let mut next_queue = VecDeque::new();
                while let Some(elem) = queue.pop_front() {
                    match (elem.borrow().left.clone(), elem.borrow().right.clone()) {
                        (None, None) => {
                            next_queue = VecDeque::new();
                            break;
                        }
                        (None, Some(right)) => next_queue.push_back(right),
                        (Some(left), None) => next_queue.push_back(left),
                        (Some(left), Some(right)) => {
                            next_queue.push_back(right);
                            next_queue.push_back(left);
                        }
                    }
                }
                queue = next_queue;
                count += 1;
            }
            count
        } else {
            0
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_111, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        assert_eq!(problem_111::Solution::min_depth(input), 2);
    }
}
