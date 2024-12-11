use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn max_path_sum_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        max_val: &mut i32,
    ) -> Option<i32> {
        let mut res = None;
        if let Some(head) = root {
            let mut tmp_max = head.borrow().val;
            let left = head.borrow().left.clone();
            let right = head.borrow().right.clone();
            let left_max = Self::max_path_sum_helper(left, max_val);
            let right_max = Self::max_path_sum_helper(right, max_val);
            if left_max.is_some() && left_max.unwrap() > 0 {
                tmp_max += left_max.unwrap();
            }
            if right_max.is_some() && right_max.unwrap() > 0 {
                tmp_max += right_max.unwrap();
            }

            *max_val = max(*max_val, tmp_max);
            res = Some(max(
                head.borrow().val,
                head.borrow().val + max(left_max.unwrap_or(0), right_max.unwrap_or(0)),
            ));
        }
        res
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("{:?}", root);
        let mut max_val = root.clone().unwrap().borrow().val;
        Self::max_path_sum_helper(root, &mut max_val);
        max_val
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_124, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        let input = TreeNode::from_vec(0, vec![Some(-3)]);
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                Some(1),
            ],
        );
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(9),
                Some(6),
                Some(-3),
                None,
                None,
                Some(-6),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(-6),
                Some(-6),
            ],
        );
        assert_eq!(problem_124::Solution::max_path_sum(input), 16);
    }
}
