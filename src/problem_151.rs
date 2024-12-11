use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = "".to_string();
        let words: Vec<&str> = s.split(" ").collect();
        for word in words.iter().rev() {
            if !word.is_empty() {
                if !res.is_empty() {
                    res.push(' ');
                };
                res.push_str(word);
            }
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_151::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        // let input = String::from("the sky is blue");
        // let output = String::from("blue is sky the");
        let input = String::from("  hello world  ");
        let output = String::from("world hello");
        assert_eq!(problem_151::Solution::reverse_words(input), output);
    }
}
