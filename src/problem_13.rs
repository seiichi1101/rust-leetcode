use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let map = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

        for (i, c) in s.chars().enumerate() {
            let num = map.get(&c).unwrap();
            let nextC = s.chars().nth(i + 1);
            let nextNum = match nextC {
                Some(nc) => map.get(&nc).unwrap(),
                None => &0,
            };
            if num >= nextNum {
                res += num
            } else {
                res -= num
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_13::{self, Solution};

    #[test]
    fn test1() {
        let input = String::from("III");
        assert_eq!(problem_13::Solution::roman_to_int(input), 3);
    }
    #[test]
    fn test2() {
        let input = String::from("LVIII");
        assert_eq!(problem_13::Solution::roman_to_int(input), 58);
    }
}
