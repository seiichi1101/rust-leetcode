use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let mut new_tree = TreeNode::new(node1.borrow().val + node2.borrow().val);
                new_tree.left =
                    Self::merge_trees(node1.borrow().left.clone(), node2.borrow().left.clone());
                new_tree.right =
                    Self::merge_trees(node1.borrow().right.clone(), node2.borrow().right.clone());
                Some(Rc::new(RefCell::new(new_tree)))
            }
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (None, None) => None,
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_617::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input1 = TreeNode::from_vec(0, vec![Some(1), Some(3), Some(2), Some(5)]);
        let input2 = TreeNode::from_vec(
            0,
            vec![Some(2), Some(1), Some(3), None, Some(4), None, Some(7)],
        );
        let output = TreeNode::from_vec(
            0,
            vec![Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)],
        );
        assert_eq!(problem_617::Solution::merge_trees(input1, input2), output);
    }
}
