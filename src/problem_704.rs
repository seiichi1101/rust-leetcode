impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mid = nums.len() / 2;
        if nums[mid] == target {
            return mid as i32;
        }
        if nums.len() == 1 {
            return -1;
        }

        if nums[mid] > target {
            Self::search(nums[..mid].to_vec(), target)
        } else {
            let res = Self::search(nums[mid..].to_vec(), target);
            if res == -1 {
                res
            } else {
                mid as i32 + res
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_704::{self};

    #[test]
    fn test1() {
        let input = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(problem_704::Solution::search(input, target), 4);
    }
}
