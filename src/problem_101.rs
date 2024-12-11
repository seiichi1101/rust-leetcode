use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::TreeNode;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut deq = VecDeque::new();
        deq.push_back(root.clone());
        deq.push_back(root.clone());
        while !deq.is_empty() {
            let node1 = deq.pop_front().unwrap();
            let node2 = deq.pop_front().unwrap();

            match (node1, node2) {
                (Some(n1), Some(n2)) => {
                    if n1.borrow().val != n2.borrow().val {
                        return false;
                    } else {
                        deq.push_back(n1.borrow().left.clone());
                        deq.push_back(n2.borrow().right.clone());
                        deq.push_back(n1.borrow().right.clone());
                        deq.push_back(n2.borrow().left.clone());
                    }
                }
                (Some(_), None) => return false,
                (None, Some(_)) => return false,
                (None, None) => {
                    continue;
                }
            }
        }
        true
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_101::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, [1, 2, 2, 3, 4, 4, 3].iter().map(|x| Some(*x)).collect());
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)],
        // );
        assert_eq!(problem_101::Solution::is_symmetric(input), true);
    }
}
