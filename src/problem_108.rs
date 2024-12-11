use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mid = nums.len() / 2;
        if nums.len() > 0 {
            let mut root = TreeNode::new(nums[mid]);
            if nums.len() > 1 {
                root.left = Self::sorted_array_to_bst(nums[..mid].to_vec());
                if nums.len() > 2 {
                    root.right = Self::sorted_array_to_bst(nums[(mid + 1)..].to_vec());
                }
            }
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_108, TreeNode};

    #[test]
    fn test1() {
        let input = vec![-10, -3, 0, 5, 9];
        let ans = TreeNode::from_vec(
            0,
            vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)],
        );
        let output = problem_108::Solution::sorted_array_to_bst(input);
        assert_eq!(output, ans)
    }
}
