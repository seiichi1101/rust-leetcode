use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        if let Some(head) = root {
            let val = head.borrow().val as f64;
            let left_or_right = if target < val {
                head.borrow().left.clone()
            } else {
                head.borrow().right.clone()
            };

            let next_val = Self::closest_value(left_or_right, target) as f64;
            if (target - val).abs() == (target - next_val).abs() {
                min(val as i32, next_val as i32)
            } else if (target - val).abs() <= (target - next_val).abs() {
                val as i32
            } else {
                next_val as i32
            }
        } else {
            i32::max_value()
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_270, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(4), Some(2), Some(5), Some(1), Some(3)]);
        let target = 3.5;
        // let input = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]);
        // let target = 0.142857;
        assert_eq!(problem_270::Solution::closest_value(input, target), 3);
    }
}
