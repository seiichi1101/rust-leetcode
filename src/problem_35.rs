impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = nums.len();
        for i in 0..nums.len() {
            if target <= nums[i] {
                res = i;
                break;
            }
        }
        res as i32
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_35::{self, Solution};

    #[test]
    fn test1() {
        let input = vec![1, 3, 5, 6];
        assert_eq!(problem_35::Solution::search_insert(input, 7), 4);
    }
}
