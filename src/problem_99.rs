use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut curr = root.clone();
        let mut x = None;
        let mut y = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;

        while !(stack.is_empty() && curr.is_none()) {
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                if let Some(p) = pred {
                    if p.borrow().val > node.borrow().val {
                        y = Some(node.clone());
                        if x.is_none() {
                            x = Some(p);
                        } else {
                            break;
                        }
                    }
                }
                pred = Some(node.clone());
                curr = node.borrow().right.clone();
            }
        }

        let mut x = x.as_ref().unwrap().borrow_mut();
        let mut y = y.as_ref().unwrap().borrow_mut();
        let tmp = x.val;
        x.val = y.val;
        y.val = tmp;
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{problem_99, TreeNode};

    #[test]
    fn test1() {
        let mut input = TreeNode::from_vec(0, vec![Some(1), Some(3), None, None, Some(2)]);
        let output = TreeNode::from_vec(0, vec![Some(3), Some(1), None, None, Some(2)]);
        problem_99::Solution::recover_tree(&mut input);
        assert_eq!(input, output);
    }
}
