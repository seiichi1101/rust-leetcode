impl Solution {
    // DP
    // pub fn climb_stairs(n: i32) -> i32 {
    //     let mut dp: Vec<i32> = vec![1];

    //     for i in 1..n {
    //         dp.push(dp.get((i - 1) as usize).unwrap_or(&1) + dp.get((i - 2) as usize).unwrap_or(&1))
    //     }
    //     dp.pop().unwrap()
    // }

    // fibonacci
    pub fn climb_stairs(n: i32) -> i32 {
        let mut pprev = 1;
        let mut prev = 2;

        if n == 1 {
            pprev
        } else if n == 2 {
            prev
        } else {
            let mut res = 0;
            for _i in 3..(n + 1) {
                res = pprev + prev;
                pprev = prev;
                prev = res;
            }
            res
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_70::{self, Solution};

    #[test]
    fn test1() {
        let input = 3;
        assert_eq!(problem_70::Solution::climb_stairs(input), 3);
    }
}
