impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x >= 0 { 1 } else { -1 };
        let mut reversed = String::new();
        for c in x.abs().to_string().chars().rev() {
            reversed.push(c);
        }
        match reversed.parse::<i32>() {
            Ok(num) => num * sign,
            Err(_) => 0,
        }
    }

    // TODO: 文字列変換しなくても数値計算だけでいける
    // https://leetcode.com/problems/reverse-integer/solution/
    // pub fn reverse(x: i32) -> i32 {    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_7::{self};

    #[test]
    fn test1() {
        let input = vec![123];
        let output = vec![321];
        for i in 0..input.len() {
            assert_eq!(problem_7::Solution::reverse(input[i]), output[i]);
        }
    }
}
