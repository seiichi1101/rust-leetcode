use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut mut_k = k;
        let mut mut_root = root;
        loop {
            while mut_root.is_some() {
                stack.push(mut_root.clone());
                mut_root = mut_root.unwrap().borrow().left.clone();
            }

            mut_root = stack.pop().unwrap();
            mut_k -= 1;
            if mut_k == 0 {
                return mut_root.unwrap().borrow().val;
            }
            mut_root = mut_root.unwrap().borrow().right.clone();
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_230, TreeNode};

    #[test]
    fn test1() {
        let root = TreeNode::from_vec(0, vec![Some(3), Some(1), Some(4), None, Some(2)]);
        let k = 1;
        let output = 1;
        assert_eq!(problem_230::Solution::kth_smallest(root, k), output);
    }
}
