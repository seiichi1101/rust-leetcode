use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if let Some(root) = root {
            let val = root.borrow().val;
            if val == target_sum && root.borrow().left.is_none() && root.borrow().right.is_none() {
                res.push(vec![val]);
            } else {
                let lefts = Self::path_sum(root.borrow().left.clone(), target_sum - val);
                let rights = Self::path_sum(root.borrow().right.clone(), target_sum - val);

                for left in lefts {
                    let mut tmp_res = vec![val];
                    tmp_res.extend(left);
                    res.push(tmp_res);
                }
                for right in rights {
                    let mut tmp_res = vec![val];
                    tmp_res.extend(right);
                    res.push(tmp_res);
                }
            }
        }

        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_113, TreeNode};

    #[test]
    fn test1() {
        let root = TreeNode::from_vec(0, vec![Some(1), Some(2)]);
        let target = 1;
        let output = vec![vec![]];
        // let root = TreeNode::from_vec(
        //     0,
        //     vec![
        //         Some(5),
        //         Some(4),
        //         Some(8),
        //         Some(11),
        //         None,
        //         Some(13),
        //         Some(4),
        //         Some(7),
        //         Some(2),
        //         None,
        //         None,
        //         None,
        //         None,
        //         Some(5),
        //         Some(1),
        //     ],
        // );
        // let target = 22;
        // let output = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        assert_eq!(problem_113::Solution::path_sum(root, target), output);
    }
}
