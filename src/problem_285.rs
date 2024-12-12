use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

impl Solution {
    pub fn _inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        ans: &mut Option<Rc<RefCell<TreeNode>>>,
        res: &mut bool,
    ) {
        if let Some(node) = &root {
            let n = node.borrow();

            Self::_inorder_successor(n.left.clone(), target.clone(), ans, res);
            if *res && ans.is_none() {
                *ans = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
            }
            if let Some(t) = &target {
                if t.borrow().val == n.val {
                    *res = true;
                }
            }
            Self::_inorder_successor(n.right.clone(), target.clone(), ans, res);
        }
    }
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        let mut bool = false;
        Self::_inorder_successor(root, p, &mut ans, &mut bool);
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        problem_285::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let root = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]);
        // let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        // let output = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = TreeNode::from_vec(
            0,
            vec![
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                None,
                Some(1),
            ],
        );
        let p = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let output = None;
        assert_eq!(problem_285::Solution::inorder_successor(root, p), output);
    }
}
