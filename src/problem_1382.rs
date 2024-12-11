use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(head) = root.clone() {
            let mut current = head.clone();
            while current.clone().borrow().right.is_some() {
                if current
                    .clone()
                    .borrow()
                    .right
                    .clone()
                    .unwrap()
                    .borrow()
                    .left
                    .is_some()
                {
                    Self::right_rotation(
                        current.clone(),
                        current.clone().borrow().right.clone().unwrap(),
                    )
                } else {
                    current = current.clone().borrow().right.clone().unwrap()
                }
            }
            println!("head: {:?}", head);
            root
        } else {
            root
        }
    }

    pub fn right_rotation(parent: Rc<RefCell<TreeNode>>, node: Rc<RefCell<TreeNode>>) {
        let tmp = node.borrow().left.clone();
        node.borrow_mut().left = tmp.clone().unwrap().borrow().right.clone();
        // tmp.unwrap().borrow_mut().right = None;
        // parent.borrow_mut().right = tmp;
        let tmp_right_tree_node = TreeNode {
            val: tmp.clone().unwrap().borrow().val,
            left: tmp.clone().unwrap().borrow().left.clone(),
            right: None,
        };
        parent.borrow_mut().right = Some(Rc::new(RefCell::new(tmp_right_tree_node)));
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_1382::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(1),
                None,
                Some(2),    
                None,
                None,
                None,
                Some(3),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
            ],
        );
        let output = TreeNode::from_vec(
            0,
            vec![Some(2), Some(1), Some(3), None, None, None, Some(4)],
        );
        assert_eq!(problem_1382::Solution::balance_bst(input), output);
    }
}
