use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn retrieve_sequence(root: Option<Rc<RefCell<TreeNode>>>, sequence: &mut Vec<i32>) {
        if let Some(node) = root {
            let n = node.borrow();
            match (n.left.clone(), n.right.clone()) {
                (None, None) => sequence.push(n.val),
                (None, Some(_)) => Self::retrieve_sequence(n.right.clone(), sequence),
                (Some(_), None) => Self::retrieve_sequence(n.left.clone(), sequence),
                (Some(_), Some(_)) => {
                    Self::retrieve_sequence(n.right.clone(), sequence);
                    Self::retrieve_sequence(n.left.clone(), sequence);
                }
            }
        }
    }
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let (mut sequence1, mut sequence2) = (vec![], vec![]);
        Self::retrieve_sequence(root1, &mut sequence1);
        Self::retrieve_sequence(root2, &mut sequence2);
        sequence1 == sequence2
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_872::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input1 = TreeNode::from_vec(
            0,
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(9),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
        );
        let input2 = TreeNode::from_vec(
            0,
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(7),
                Some(4),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(9),
                Some(8),
            ],
        );
        let output = true;
        assert_eq!(problem_872::Solution::leaf_similar(input1, input2), output);
    }
}
