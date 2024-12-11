use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn is_valid(
        min_val: Option<i32>,
        max_val: Option<i32>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(root) = root {
            match (max_val, min_val) {
                (None, None) => return false,
                (None, Some(min_val)) => {
                    if root.borrow().val <= min_val {
                        return false;
                    }
                }
                (Some(max_val), None) => {
                    if root.borrow().val >= max_val {
                        return false;
                    }
                }
                (Some(max_val), Some(min_val)) => {
                    if root.borrow().val >= max_val || root.borrow().val <= min_val {
                        return false;
                    }
                }
            }
            Self::is_valid(
                min_val,
                Some(min(max_val.unwrap_or(i32::MAX), root.borrow().val)),
                root.borrow().left.clone(),
            ) && Self::is_valid(
                Some(max(min_val.unwrap_or(i32::MIN), root.borrow().val)),
                max_val,
                root.borrow().right.clone(),
            )
        } else {
            true
        }
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::is_valid(None, Some(root.borrow().val), root.borrow().left.clone())
                && Self::is_valid(Some(root.borrow().val), None, root.borrow().right.clone())
        } else {
            true
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{problem_98, TreeNode};

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]);
        // let input = TreeNode::from_vec(0, vec![Some(2), Some(2), Some(2)]);
        // let input = TreeNode::from_vec(0, vec![Some(-2147483648), None, Some(2147483647)]);
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
        // );
        let input = TreeNode::from_vec(
            0,
            vec![
                Some(3),
                Some(1),
                Some(5),
                Some(0),
                Some(2),
                Some(4),
                Some(6),
            ],
        );
        // let input = Some(Rc::new(RefCell::new(TreeNode {
        //     val: 3,
        //     left: None,
        //     right: Some(Rc::new(RefCell::new(TreeNode {
        //         val: 30,
        //         left: Some(Rc::new(RefCell::new(TreeNode {
        //             val: 10,
        //             left: None,
        //             right: Some(Rc::new(RefCell::new(TreeNode {
        //                 val: 15,
        //                 left: None,
        //                 right: Some(Rc::new(RefCell::new(TreeNode {
        //                     val: 45,
        //                     left: None,
        //                     right: None,
        //                 }))),
        //             }))),
        //         }))),
        //         right: None,
        //     }))),
        // })));

        println!("input: {:?}", input);
        assert_eq!(problem_98::Solution::is_valid_bst(input), false);
    }
}
