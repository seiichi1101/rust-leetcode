impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut min = prices[0];
        let mut max = prices[0];

        for i in 1..prices.len() {
            if prices[i - 1] > prices[i] {
                if min > prices[i] {
                    min = prices[i];
                    max = prices[i];
                }
            } else {
                max = prices[i];
            }

            if max - min > res {
                res = max - min;
            }
        }

        res
    }
}

struct Solution;

#[cfg(test)]
pub mod tests {
    use crate::problem_121::{self};

    #[test]
    fn test1() {
        // let input = vec![7, 1, 5, 3, 6, 4];
        let input = vec![7, 6, 4, 3, 1];
        assert_eq!(problem_121::Solution::max_profit(input), 0);
    }
}
