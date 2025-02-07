use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    // TODO: better to reuse hashmap as a desired target
    pub fn find(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64, dp: Vec<i64>) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let n_val = n.val as i64;
            let mut res = 0;
            let mut new_dp = vec![n_val];
            if n_val == target_sum {
                res += 1;
            }
            for i in dp {
                if i + n_val == target_sum {
                    res += 1;
                }
                new_dp.push(i + n_val);
            }
            res + Self::find(n.left.clone(), target_sum, new_dp.clone())
                + Self::find(n.right.clone(), target_sum, new_dp.clone())
        } else {
            0
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::find(root, target_sum as i64, vec![])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_437::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let root = TreeNode::from_vec(
        //     0,
        //     vec![
        //         Some(10),
        //         Some(5),
        //         Some(-3),
        //         Some(3),
        //         Some(2),
        //         None,
        //         Some(11),
        //         Some(3),
        //         Some(-2),
        //         None,
        //         Some(1),
        //     ],
        // );
        // let target_sum = 8;
        // let output = 3;
        let root = TreeNode::from_vec(
            0,
            vec![
                Some(1000000000),
                Some(1000000000),
                None,
                Some(294967296),
                None,
                None,
                None,
                Some(1000000000),
                None,
                None,
                None,
                None,
                None,
                Some(1000000000),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(1000000000),
            ],
        );
        let target_sum = 0;
        let output = 0;
        assert_eq!(problem_437::Solution::path_sum(root, target_sum), output);
    }
}
