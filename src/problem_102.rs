use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        struct Elem {
            level: i32,
            node: Option<Rc<RefCell<TreeNode>>>,
        }
        let mut queue = VecDeque::new();
        queue.push_back(Elem {
            level: 0,
            node: root,
        });

        let mut current_level = 0;
        let mut res = vec![];
        let mut tmp_res = vec![];
        while !queue.is_empty() {
            if let Some(next) = queue.pop_front() {
                if let Some(next_node) = next.node {
                    if next.level > current_level {
                        res.push(tmp_res);
                        tmp_res = vec![];
                        current_level = next.level;
                    }
                    tmp_res.push(next_node.borrow().val);
                    queue.push_back(Elem {
                        level: current_level + 1,
                        node: next_node.borrow().left.clone(),
                    });
                    queue.push_back(Elem {
                        level: current_level + 1,
                        node: next_node.borrow().right.clone(),
                    });
                }
            }
        }

        if !tmp_res.is_empty() {
            res.push(tmp_res);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_102, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        let output = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(problem_102::Solution::level_order(input), output);
    }
}
