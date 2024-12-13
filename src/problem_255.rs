impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        if preorder.is_empty() {
            return true;
        }

        let mut min: Option<i32> = None;
        let mut stack = vec![];
        for ele in preorder {
            while !stack.is_empty() && *stack.last().unwrap() < ele {
                min = stack.pop();
            }

            if min.is_some() && ele < min.unwrap() {
                return false;
            }
            stack.push(ele);
        }

        true
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_255::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let input = vec![5, 2, 1, 3, 6];
        let output = true;
        // let input = vec![5, 2, 6, 1, 3];
        // let output = false;
        // let input = vec![1, 3, 4, 2];
        // let output = false;
        assert_eq!(problem_255::Solution::verify_preorder(input), output);
    }
}
