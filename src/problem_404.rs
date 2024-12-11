use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(head) = root {
            let left = head.borrow().left.clone();
            let right = head.borrow().right.clone();
            let l_val = if left.is_some()
                && left.clone().unwrap().borrow().left.is_none()
                && left.clone().unwrap().borrow().right.is_none()
            {
                left.unwrap().borrow().val
            } else {
                Self::sum_of_left_leaves(left)
            };
            let r_val = Self::sum_of_left_leaves(right);
            l_val + r_val
        } else {
            0
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_404, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        assert_eq!(problem_404::Solution::sum_of_left_leaves(input), 24);
    }
}
