use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn post_order(root: Option<Rc<RefCell<TreeNode>>>, dp: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let l = Self::post_order(n.left.clone(), dp);
            let r = Self::post_order(n.right.clone(), dp);
            let k = n.val + l + r;
            if let Some(v) = dp.get(&k) {
                dp.insert(k, v + 1);
            } else {
                dp.insert(k, 1);
            }
            k
        } else {
            0
        }
    }
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut dp = HashMap::new();
        Self::post_order(root, &mut dp);
        if let Some(max) = dp.values().max() {
            for (k, v) in dp.iter() {
                if max == v {
                    res.push(*k);
                }
            }
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_508::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(0, vec![Some(5), Some(2), Some(-3)]);
        // let output = vec![2, -3, 4];
        let input = TreeNode::from_vec(0, vec![Some(5), Some(14), None, Some(1)]);
        let output = vec![1, 15, 20];
        assert_eq!(problem_508::Solution::find_frequent_tree_sum(input), output);
    }
}
