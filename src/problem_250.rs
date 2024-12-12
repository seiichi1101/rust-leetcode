use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn subtrees(
        count: &mut i32,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Result<i32, String>> {
        if let Some(node) = root {
            let n = node.borrow();
            let left = Self::subtrees(count, n.left.clone());
            let right = Self::subtrees(count, n.right.clone());
            match (left, right) {
                (None, None) => {
                    *count += 1;
                    return Some(Ok(n.val));
                }
                (None, Some(r)) => {
                    if r.is_ok() && r.unwrap() == n.val {
                        *count += 1;
                        return Some(Ok(n.val));
                    }
                }
                (Some(l), None) => {
                    if l.is_ok() && l.unwrap() == n.val {
                        *count += 1;
                        return Some(Ok(n.val));
                    }
                }
                (Some(l), Some(r)) => {
                    if l.is_ok() && r.is_ok() && l.unwrap() == n.val && r.unwrap() == n.val {
                        *count += 1;
                        return Some(Ok(n.val));
                    }
                }
            }
            Some(Err("no more uni value".to_string()))
        } else {
            None
        }
    }

    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::subtrees(&mut res, root);
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_250::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(5), Some(1), Some(5), Some(5), Some(5), None, Some(5)],
        // );
        // let output = 4;
        let input = TreeNode::from_vec(
            0,
            vec![Some(1), Some(1), Some(1), Some(5), Some(5), None, Some(5)],
        );
        let output = 3;
        assert_eq!(problem_250::Solution::count_unival_subtrees(input), output);
    }
}
