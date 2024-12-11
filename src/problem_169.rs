impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = None;

        for n in nums {
            if count == 0 {
                candidate = Some(n);
            }

            if candidate == Some(n) {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate.unwrap()
    }
}

struct Solution;

#[cfg(test)]
pub mod tests {
    use crate::problem_169::{self};

    #[test]
    fn test1() {
        let input = vec![2, 2, 1];
        assert_eq!(problem_169::Solution::majority_element(input), 2);
    }
}
