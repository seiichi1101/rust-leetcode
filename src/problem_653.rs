use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            if let Some(Some(node)) = queue.pop_front() {
                let n = node.borrow();
                if set.contains(&(k - n.val)) {
                    return true;
                }
                set.insert(n.val);
                queue.push_back(n.left.clone());
                queue.push_back(n.right.clone());
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_653::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
        // );
        // let k = 9;
        let input = TreeNode::from_vec(0, vec![Some(2), Some(1), Some(3)]);
        let k = 4;
        assert_eq!(problem_653::Solution::find_target(input, k), true);
    }
}
