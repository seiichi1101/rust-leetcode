use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        if let Some(r) = root {
            r.borrow_mut().val = 1;
            let mut queue = VecDeque::from([r]);
            while !queue.is_empty() {
                let ele = queue.pop_front();
                if let Some(e) = ele {
                    if e.borrow().val > res {
                        res = e.borrow().val;
                    }
                    let left = e.borrow().left.clone();
                    if let Some(l) = left {
                        l.borrow_mut().val = e.borrow().val + 1;
                        queue.push_back(l);
                    }
                    let right = e.borrow().right.clone();
                    if let Some(r) = right {
                        r.borrow_mut().val = e.borrow().val + 1;
                        queue.push_back(r);
                    }
                }
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_104, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        assert_eq!(problem_104::Solution::max_depth(input), 3);
    }
}
