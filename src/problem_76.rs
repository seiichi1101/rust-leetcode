use std::collections::HashMap;

// TLE
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut res = String::from("");
        let mut start = 0;
        let mut end = 0;
        let mut words = HashMap::new();
        for c in t.chars() {
            *words.entry(c).or_insert(0) += 1;
        }

        #[derive(Debug, PartialEq)]
        enum MoveStatus {
            moveStart,
            moveEnd,
        }

        let mut moveStatus = MoveStatus::moveEnd;

        while start <= s.len() && end <= s.len() {
            if moveStatus == MoveStatus::moveEnd {
                if words.values().any(|&val| val > 0) {
                    if let Some(c) = s.chars().nth(end) {
                        if words.contains_key(&c) {
                            *words.entry(c).or_default() -= 1;
                        }
                    }
                    end += 1;
                } else {
                    moveStatus = MoveStatus::moveStart;
                }
            } else if moveStatus == MoveStatus::moveStart {
                if words.values().all(|&val| val <= 0) {
                    if res.is_empty() || end - start < res.len() {
                        res = s.chars().skip(start).take(end - start).collect();
                    }
                    if let Some(c) = s.chars().nth(start) {
                        if words.contains_key(&c) {
                            *words.entry(c).or_default() += 1;
                        }
                    }
                    start += 1;
                } else {
                    moveStatus = MoveStatus::moveEnd;
                }
            }
        }
        println!("res: {:?}, words: {:?}", res, words);
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_76::{self, Solution};
    #[test]
    fn test0() {
        let s = String::from("a");
        let t = String::from("aa");
        assert_eq!(problem_76::Solution::min_window(s, t), String::from(""));
    }

    #[test]
    fn test1() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        assert_eq!(problem_76::Solution::min_window(s, t), String::from("BANC"));
    }

    #[test]
    fn test2() {
        let s = String::from("ABAACBAB");
        let t = String::from("ABC");
        assert_eq!(problem_76::Solution::min_window(s, t), String::from("ACB"));
    }
}
