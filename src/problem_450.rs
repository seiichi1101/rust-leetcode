use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    // find new root
    // 後継者(successor)探し
    pub fn successor(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        let new_root = root.unwrap().borrow().right.clone();
        let mut res = None;
        let mut current = new_root;
        while let Some(node) = current.clone() {
            let n = node.borrow();
            res = Some(n.val);
            current = n.left.clone();
        }
        res
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let n = node.borrow();
            if n.val == key {
                match (n.left.clone(), n.right.clone()) {
                    (None, None) => None,
                    (None, Some(r)) => Some(r),
                    (Some(l), None) => Some(l),
                    (Some(l), Some(r)) => {
                        let new_val = Self::successor(root);
                        Some(Rc::new(RefCell::new(TreeNode {
                            val: new_val.unwrap(),
                            left: Some(l),
                            right: Self::delete_node(Some(r), new_val.unwrap()),
                        })))
                    }
                }
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: n.val,
                    left: Self::delete_node(n.left.clone(), key),
                    right: Self::delete_node(n.right.clone(), key),
                })))
            }
        } else {
            None
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_450::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
        // );
        // let key = 3;
        // let output = TreeNode::from_vec(
        //     0,
        //     vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7)],
        // );
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(50),
                Some(30),
                Some(70),
                None,
                Some(40),
                Some(60),
                Some(80),
            ],
        );
        let key = 50;
        let output = TreeNode::from_vec(
            0,
            vec![Some(60), Some(30), Some(70), None, Some(40), None, Some(80)],
        );

        assert_eq!(problem_450::Solution::delete_node(input, key), output);
    }
}
