impl Solution {
    // Recursive
    // pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    //     if num_rows == 1 {
    //         return vec![vec![1]];
    //     }
    //     let mut res = Self::generate(num_rows - 1);
    //     let prev = res.last().unwrap();

    //     let mut tmp = vec![1];
    //     for i in 0..prev.len() - 1 {
    //         tmp.push(prev.get(i).unwrap() + prev.get(i + 1).unwrap());
    //     }
    //     tmp.push(1);

    //     res.push(tmp);
    //     return res;
    // }

    // DP
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        for i in 1..num_rows {
            let mut tmp = vec![];
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    tmp.push(1);
                } else {
                    let prev = res.get((i - 1) as usize).unwrap();
                    let l = prev.get((j - 1) as usize).unwrap();
                    let r = prev.get((j) as usize).unwrap();
                    tmp.push(l + r);
                }
            }
            res.push(tmp);
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_118::{self, Solution};

    #[test]
    fn test1() {
        let input = 5;
        let output = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(problem_118::Solution::generate(input), output);
    }
}
