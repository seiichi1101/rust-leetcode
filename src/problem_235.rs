use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            if root == p || root == q {
                root
            } else {
                let l =
                    Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
                let r =
                    Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());
                match (&l, &r) {
                    (None, None) => None,
                    (None, Some(_)) => r,
                    (Some(_), None) => l,
                    (Some(_), Some(_)) => root,
                }
            }
        } else {
            None
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_235, TreeNode};

    #[test]
    fn test1() {
        let root = TreeNode::from_vec(
            0,
            vec![
                Some(6),
                Some(2),
                Some(8),
                Some(0),
                Some(4),
                Some(7),
                Some(9),
                None,
                None,
                Some(3),
                Some(5),
            ],
        );
        let (p, q, o) = (
            root.clone().unwrap().borrow().left.clone(),
            root.clone()
                .unwrap()
                .borrow()
                .left
                .clone()
                .unwrap()
                .borrow()
                .right
                .clone(),
            root.clone().unwrap().borrow().left.clone(),
        );
        assert_eq!(problem_235::Solution::lowest_common_ancestor(root, p, q), o);
    }
}
