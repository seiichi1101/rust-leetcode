impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for i in (0..nums.len()).rev() {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_283::{self};

    #[test]
    fn test1() {
        let mut input = vec![0, 1, 0, 3, 12];
        let output = vec![1, 3, 12, 0, 0];
        // let mut input = vec![0, 0, 1];
        // let output = vec![1, 0, 0];
        problem_283::Solution::move_zeroes(&mut input);
        assert_eq!(input, output);
    }
}
