use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut longest = 1;
        fn helper(
            root: Option<Rc<RefCell<TreeNode>>>,
            parent: Option<Rc<RefCell<TreeNode>>>,
            mut tmp_longest: i32,
            longest: &mut i32,
        ) {
            if let Some(node) = root {
                if let Some(par) = parent {
                    let n = node.borrow();
                    let p = par.borrow();
                    if n.val - p.val == 1 {
                        tmp_longest += 1;
                    } else {
                        tmp_longest = 1;
                    }
                    *longest = max(*longest, tmp_longest);
                }
                helper(
                    node.borrow().left.clone(),
                    Some(node.clone()),
                    tmp_longest,
                    longest,
                );
                helper(
                    node.borrow().right.clone(),
                    Some(node.clone()),
                    tmp_longest,
                    longest,
                );
            }
        };
        helper(root, None, 1, &mut longest);
        longest
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_298::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), Some(2), Some(2)]);
        let output = 2;
        // let input = TreeNode::from_vec(
        //     0,
        //     vec![
        //         Some(1),
        //         None,
        //         Some(3),
        //         None,
        //         None,
        //         Some(2),
        //         Some(4),
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         None,
        //         Some(5),
        //     ],
        // );
        // let output = 3;
        assert_eq!(problem_298::Solution::longest_consecutive(input), output);
    }
}
