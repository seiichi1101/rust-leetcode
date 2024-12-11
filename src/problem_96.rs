impl Solution {
    // brute forde
    // pub fn num_trees(n: i32) -> i32 {
    //     if n == 0 || n == 1 {
    //         1
    //     } else {
    //         let mut res = 0;
    //         for i in 1..=n {
    //             let left = Self::num_trees(i - 1);
    //             let right = Self::num_trees(n - i);
    //             res += left * right
    //         }
    //         res
    //     }
    // }
    // dp
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_96::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        assert_eq!(problem_96::Solution::num_trees(3), 5);
    }
}
