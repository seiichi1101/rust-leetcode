use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    // O(logN) Binary Search in Binary Search
    // pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {}

    // O(N)
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut queue = VecDeque::new();
        if let Some(head) = root {
            queue.push_back(head);
            while !queue.is_empty() {
                let elem = queue.pop_front();
                if let Some(e) = elem {
                    let left = e.borrow().left.clone();
                    let right = e.borrow().right.clone();
                    match (left, right) {
                        (None, None) => {}
                        (None, Some(r)) => queue.push_back(r),
                        (Some(l), None) => queue.push_back(l),
                        (Some(l), Some(r)) => {
                            queue.push_back(l);
                            queue.push_back(r);
                        }
                    };
                    res += 1;
                }
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_222, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
        );
        assert_eq!(problem_222::Solution::count_nodes(input), 6);
    }
}
