use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        struct Elem {
            node: Option<Rc<RefCell<TreeNode>>>,
            key: i32,
        }
        let mut queue = VecDeque::new();
        queue.push_back(Elem { node: root, key: 0 });

        while !queue.is_empty() {
            if let Some(elem) = queue.pop_front() {
                if let Some(node) = elem.node {
                    let entry = map.entry(elem.key).or_insert_with(Vec::new);
                    entry.push(node.borrow().val);

                    queue.push_back(Elem {
                        node: node.borrow().left.clone(),
                        key: elem.key - 1,
                    });
                    queue.push_back(Elem {
                        node: node.borrow().right.clone(),
                        key: elem.key + 1,
                    });
                }
            }
        }

        let mut keys: Vec<_> = map.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(val) = map.get(&key) {
                res.push(val.clone());
            }
        }

        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_314::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        let output = vec![vec![9], vec![3, 15], vec![20], vec![7]];
        assert_eq!(problem_314::Solution::vertical_order(input), output);
    }
}
