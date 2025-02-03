use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn postorder(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) -> usize {
        if let Some(node) = root {
            let n = node.borrow();

            let l = Self::postorder(n.left.clone(), res);
            let r = Self::postorder(n.right.clone(), res);
            let i = max(l, r);
            if res.get(i).is_some() {
                res[i].push(n.val);
            } else {
                res.push(vec![n.val]);
            }
            i + 1
        } else {
            0
        }
    }
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::postorder(root, &mut res);
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_366::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let output = vec![vec![4, 5, 3], vec![2], vec![1]];
        assert_eq!(problem_366::Solution::find_leaves(input), output);
    }
}
