use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn upside_down_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            header: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if root.is_some() {
                let node = root.clone().unwrap();
                let mut n = node.borrow_mut();
                if n.left.is_some() {
                    let new_root = dfs(n.left.clone(), header);
                    n.left.clone().unwrap().borrow_mut().left = n.right.clone();
                    n.left.clone().unwrap().borrow_mut().right = Some(node.clone());
                    n.left = None;
                    n.right = None;
                    return new_root;
                } else if header.is_none() {
                    *header = root.clone();
                    return None;
                }
            }
            None
        }
        let mut header = None;
        dfs(root, &mut header);
        header
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_156::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let output = TreeNode::from_vec(
            0,
            vec![Some(4), Some(5), Some(2), None, None, Some(3), Some(1)],
        );
        assert_eq!(
            problem_156::Solution::upside_down_binary_tree(input),
            output
        );
    }
}
