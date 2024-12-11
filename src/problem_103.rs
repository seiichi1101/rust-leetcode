use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        struct Elem {
            level: i32,
            node: Option<Rc<RefCell<TreeNode>>>,
        }
        let mut res = vec![];
        let mut queue = VecDeque::new();
        let mut current_level = 0;
        queue.push_back(Elem {
            level: current_level,
            node: root,
        });
        let mut tmp_nodes = vec![];
        while !queue.is_empty() {
            if let Some(elem) = queue.pop_front() {
                if let Some(node) = elem.node {
                    if elem.level > current_level {
                        if (current_level % 2) > 0 {
                            tmp_nodes.reverse()
                        }
                        res.push(tmp_nodes);
                        tmp_nodes = vec![];
                        current_level = elem.level;
                    }
                    tmp_nodes.push(node.borrow().val);

                    queue.push_back(Elem {
                        level: current_level + 1,
                        node: node.borrow().left.clone(),
                    });
                    queue.push_back(Elem {
                        level: current_level + 1,
                        node: node.borrow().right.clone(),
                    });
                }
            }
        }
        if !tmp_nodes.is_empty() {
            if (current_level % 2) > 0 {
                tmp_nodes.reverse()
            }
            res.push(tmp_nodes);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_103, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        let output = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(problem_103::Solution::zigzag_level_order(input), output);
    }

    #[test]
    fn test2() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3)]);
        let output = vec![vec![1], vec![3, 2]];
        assert_eq!(problem_103::Solution::zigzag_level_order(input), output);
    }
}
