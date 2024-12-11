use crate::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn all_possible_bst(
        start: i32,
        end: i32,
        mut memo: HashMap<String, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut res = vec![];

        if start > end {
            res.push(None);
            return res;
        }

        let key = format!("{},{}", start, end);
        if memo.contains_key(&key) {
            return memo.get(&key).unwrap().to_vec();
        }

        for i in start..=end {
            let left_sub_trees = Self::all_possible_bst(start, i - 1, memo.clone());
            let right_sub_trees = Self::all_possible_bst(i + 1, end, memo.clone());
            for left in left_sub_trees.iter() {
                for right in right_sub_trees.iter() {
                    let root = TreeNode {
                        val: i,
                        left: left.clone(),
                        right: right.clone(),
                    };
                    res.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }
        memo.insert(key, res.clone());
        res
    }
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let memo: HashMap<String, Vec<Option<Rc<RefCell<TreeNode>>>>> = HashMap::new();
        Self::all_possible_bst(1, n, memo)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_95::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let outputs = vec![
            TreeNode::from_vec(0, vec![Some(1), None, Some(2), None, Some(3), None]),
            TreeNode::from_vec(0, vec![Some(1), None, Some(3), Some(2)]),
            TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]),
            TreeNode::from_vec(0, vec![Some(3), Some(1), None, None, Some(2)]),
            TreeNode::from_vec(0, vec![Some(3), Some(2), None, Some(1)]),
        ];
        assert_eq!(problem_95::Solution::generate_trees(3), outputs);
    }
}
