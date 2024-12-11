impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            res ^= n
        }
        res
    }
}

struct Solution;

#[cfg(test)]
pub mod tests {
    use crate::problem_136::{self};

    #[test]
    fn test1() {
        let input = vec![2, 2, 1];
        assert_eq!(problem_136::Solution::single_number(input), 1);
    }
}
