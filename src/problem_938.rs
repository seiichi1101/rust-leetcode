use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut res = 0;
        if let Some(head) = root {
            let head_val = head.borrow().val;
            let left = head.borrow().left.clone();
            let right = head.borrow().right.clone();
            if head_val >= low {
                res += Self::range_sum_bst(left, low, high);
            }
            if head_val <= high {
                res += Self::range_sum_bst(right, low, high);
            }
            if low <= head_val && head_val <= high {
                res += head_val
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_938, TreeNode};

    #[test]
    fn test1() {
        let root = TreeNode::from_vec(
            0,
            vec![
                Some(10),
                Some(5),
                Some(15),
                Some(3),
                Some(7),
                None,
                Some(18),
            ],
        );
        let low = 7;
        let high = 15;
        assert_eq!(problem_938::Solution::range_sum_bst(root, low, high), 32);
    }
}
