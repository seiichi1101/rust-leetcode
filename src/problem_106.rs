use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn gen_tree_node(
        inorder: Vec<i32>,
        postorder: &mut Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        let val = postorder.pop().unwrap();
        let inorder_mid = inorder.iter().position(|&x| x == val).unwrap();
        let right_node = Self::gen_tree_node(inorder[inorder_mid + 1..].to_vec(), postorder);
        let left_node = Self::gen_tree_node(inorder[..inorder_mid].to_vec(), postorder);
        let root = TreeNode {
            val,
            left: left_node.clone(),
            right: right_node.clone(),
        };
        Some(Rc::new(RefCell::new(root)))
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::gen_tree_node(inorder, &mut postorder.clone())
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_106, TreeNode};

    #[test]
    fn test1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let output = TreeNode::from_vec(
            0,
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        );
        assert_eq!(
            problem_106::Solution::build_tree(inorder, postorder),
            output
        );
    }
}
