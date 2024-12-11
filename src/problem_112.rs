use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut queue = VecDeque::new();
        struct Elem {
            sum: i32,
            node: Rc<RefCell<TreeNode>>,
        }
        queue.push_back(Elem {
            sum: 0,
            node: root.unwrap(),
        });
        while !queue.is_empty() {
            if let Some(elem) = queue.pop_front() {
                let next_val = elem.sum + elem.node.borrow().val;
                match (
                    elem.node.borrow().left.clone(),
                    elem.node.borrow().right.clone(),
                ) {
                    (None, None) => {
                        if next_val == target_sum {
                            return true;
                        }
                    }
                    (Some(left), None) => queue.push_back(Elem {
                        sum: next_val,
                        node: left,
                    }),
                    (None, Some(right)) => queue.push_back(Elem {
                        sum: next_val,
                        node: right,
                    }),
                    (Some(left), Some(right)) => {
                        queue.push_back(Elem {
                            sum: next_val,
                            node: left,
                        });
                        queue.push_back(Elem {
                            sum: next_val,
                            node: right,
                        });
                    }
                }
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_112, TreeNode};

    #[test]
    fn test1() {
        let tree = TreeNode::from_vec(
            0,
            vec![
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                None,
                Some(1),
            ],
        );
        let target_sum = 22;
        assert_eq!(problem_112::Solution::has_path_sum(tree, target_sum), true);
    }
}
