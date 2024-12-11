use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // match (p, q) {
        //     (None, None) => true,
        //     (None, Some(_)) => false,
        //     (Some(_), None) => false,
        //     (Some(_p), Some(_q)) => {
        //         if _p.borrow().val != _q.borrow().val {
        //             false
        //         } else {
        //             return Self::is_same_tree(_p.borrow().left.clone(), _q.borrow().left.clone())
        //                 && Self::is_same_tree(
        //                     _p.borrow().right.clone(),
        //                     _q.borrow().right.clone(),
        //                 );
        //         }
        //     }
        // }
        println!("{:?} {:?}", p, q);
        p == q
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{problem_100, TreeNode};

    #[test]
    fn test1() {
        let tree_node1 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        }));
        let tree_node2 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        }));
        assert!(Some(tree_node1) == Some(tree_node2));

        let p = TreeNode::from_vec(0, vec![Some(3), Some(0), Some(1)]);
        let q = TreeNode::from_vec(0, vec![Some(3), Some(0), Some(1)]);
        assert_eq!(problem_100::Solution::is_same_tree(p, q), true);
    }
}
