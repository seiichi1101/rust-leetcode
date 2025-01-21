use std::cell::RefCell;
use std::cmp::{max, min};
use std::i32;
use std::rc::Rc;

use crate::TreeNode;

struct Response {
    min: i32,
    max: i32,
    size: i32,
}

impl Solution {
    pub fn find(root: Option<Rc<RefCell<TreeNode>>>) -> Response {
        if let Some(node) = root {
            let n = node.borrow();
            let left = Self::find(n.left.clone());
            let right = Self::find(n.right.clone());
            if left.max < n.val && n.val < right.min {
                Response {
                    min: min(n.val, left.min),
                    max: max(n.val, right.max),
                    size: left.size + right.size + 1,
                }
            } else {
                Response {
                    min: i32::MIN,
                    max: i32::MAX,
                    size: max(left.size, right.size),
                }
            }
        } else {
            Response {
                min: i32::MAX,
                max: i32::MIN,
                size: 0,
            }
        }
    }
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find(root).size
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_333::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(10), Some(5), Some(15), Some(1), Some(8), None, Some(7)],
        );
        let output = 3;
        // let input = TreeNode::from_vec(0, vec![Some(3), Some(2), Some(4), None, None, Some(1)]);
        // let output = 2;
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![
        //         Some(4),
        //         Some(2),
        //         Some(7),
        //         Some(2),
        //         Some(3),
        //         Some(5),
        //         None,
        //         Some(2),
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         Some(1),
        //     ],
        // );
        // let output = 2;
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(1), Some(3), Some(2), Some(4), None, None, Some(5)],
        // );
        // let output = 2;
        assert_eq!(problem_333::Solution::largest_bst_subtree(input), output);
    }
}
