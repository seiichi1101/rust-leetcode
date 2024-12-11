use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set = HashSet::new();
        let mut nums = nums.clone();
        nums.sort();
        for i in 0..nums.len() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    set.insert(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        set.iter().cloned().collect()
    }
}
struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_15::{self};

    #[test]
    fn test1() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
        let input = vec![3, 0, -2, -1, 1, 2];
        let output = [[-2, -1, 3], [-1, 0, 1], [-2, 0, 2]];
        assert_eq!(problem_15::Solution::three_sum(input), output);
    }
}
