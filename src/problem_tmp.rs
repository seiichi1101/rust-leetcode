impl Solution {
    pub fn tmp(nums: Vec<i32>) -> i32 {
        nums.iter().sum::<i32>()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_tmp::{self};

    #[test]
    fn test1() {
        let input = vec![3, 0, 1];
        let output = 4;
        assert_eq!(problem_tmp::Solution::tmp(input), output);
    }
}
