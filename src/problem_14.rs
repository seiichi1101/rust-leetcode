impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::from("");
        let base = strs.get(0).unwrap();

        for i in 1..base.len() + 1 {
            let prefix: String = base.chars().take(i).collect();
            let mut flag = true;
            for s in strs[1..].iter() {
                if !s.starts_with(&prefix) {
                    flag = false;
                    break;
                }
            }

            if flag {
                res = prefix;
            }
        }
        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_14::{self, Solution};

    #[test]
    fn test1() {
        let input = vec!["flower".into(), "flow".into(), "flight".into()];
        assert_eq!(problem_14::Solution::longest_common_prefix(input), "fl");
    }

    #[test]
    fn test2() {
        let input = vec!["a".into()];
        assert_eq!(problem_14::Solution::longest_common_prefix(input), "a");
    }

    #[test]
    fn test3() {
        let input = vec!["c".into(), "acc".into(), "ccc".into()];
        assert_eq!(problem_14::Solution::longest_common_prefix(input), "");
    }
}
