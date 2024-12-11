use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n1 = word1.chars().count();
        let n2 = word2.chars().count();
        let mut res = 0;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n2 + 1]; n1 + 1];
        for i in 1..=n1 {
            dp[i][0] = i as i32;
        }
        for j in 1..=n2 {
            dp[0][j] = j as i32;
        }
        for (i, c1) in word1.chars().enumerate() {
            for (j, c2) in word2.chars().enumerate() {
                let x = if c1 == c2 { dp[i][j] } else { dp[i][j] + 1 };
                let v = min(x, min(dp[i][j + 1] + 1, dp[i + 1][j] + 1));
                dp[i + 1][j + 1] = v;
            }
        }

        dp[n1][n2]
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_72::{self, Solution};

    #[test]
    fn test1() {
        let word1 = String::from("horse");
        let word2 = String::from("ros");
        assert_eq!(problem_72::Solution::min_distance(word1, word2), 3);
    }
}
