use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = vec![];
        if let Some(head) = root {
            let val = head.borrow().val;
            let left = head.borrow().left.clone();
            let right = head.borrow().right.clone();

            match (left.clone(), right.clone()) {
                (None, None) => {
                    res.push(val.to_string());
                }
                (None, Some(_)) => {
                    let right_res = Self::binary_tree_paths(right);
                    res.extend(right_res.iter().map(|x| format!("{}->{}", val, x)));
                }
                (Some(_), None) => {
                    let left_res = Self::binary_tree_paths(left);
                    res.extend(left_res.iter().map(|x| format!("{}->{}", val, x)));
                }
                (Some(_), Some(_)) => {
                    let right_res = Self::binary_tree_paths(right);
                    res.extend(right_res.iter().map(|x| format!("{}->{}", val, x)));
                    let left_res = Self::binary_tree_paths(left);
                    res.extend(left_res.iter().map(|x| format!("{}->{}", val, x)));
                }
            }
        };
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_257, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(3), None, Some(5)]);
        let output = vec!["1->2->5", "1->3"];
        assert_eq!(problem_257::Solution::binary_tree_paths(input), output);
    }
}
