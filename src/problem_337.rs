use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Result {
    robbed: i32,
    not_robbed: i32,
}

use crate::TreeNode;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> Result {
            if let Some(node) = root {
                let n = node.borrow();
                let left = helper(n.left.clone());
                let right = helper(n.right.clone());
                Result {
                    robbed: n.val + left.not_robbed + right.not_robbed,
                    not_robbed: max(left.robbed, left.not_robbed)
                        + max(right.robbed, right.not_robbed),
                }
            } else {
                Result {
                    robbed: 0,
                    not_robbed: 0,
                }
            }
        }
        let res = helper(root);
        max(res.robbed, res.not_robbed)
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_337::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(3), Some(2), Some(3), None, Some(3), None, Some(1)],
        // );
        // let output = 7;
        // let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3)]);
        // let output = 5;
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(4), Some(1), None, Some(2), None, None, None, Some(3)],
        // );
        // let output = 7;
        let input = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3), None, Some(4)]);
        let output = 7;
        assert_eq!(problem_337::Solution::rob(input), output);
    }
}
