use std::collections::HashSet;

impl Solution {
    // O(N+M) Space Solution
    // pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    //     let mut setN = HashSet::new();
    //     let mut setM = HashSet::new();

    //     for i in 0..matrix.len() {
    //         for j in 0..matrix[0].len() {
    //             if matrix[i][j] == 0 {
    //                 setN.insert(i);
    //                 setM.insert(j);
    //             }
    //         }
    //     }

    //     for item in &setN {
    //         for j in 0..matrix[0].len() {
    //             matrix[*item][j] = 0;
    //         }
    //     }
    //     for item in &setM {
    //         for i in 0..matrix.len() {
    //             matrix[i][*item] = 0;
    //         }
    //     }
    //     // println!("N: {:?}, M:{:?}", setN, setM);
    // }

    // O(1) Space Solution
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut flagN = false;
        let mut flagM = false;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i == 0 && matrix[i][j] == 0 {
                    flagN = true;
                }
                if j == 0 && matrix[i][j] == 0 {
                    flagM = true;
                }
                if i > 0 && j > 0 && matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..matrix.len() {
            if matrix[i][0] == 0 {
                for j in 1..matrix[i].len() {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..matrix[0].len() {
            if matrix[0][j] == 0 {
                for i in 1..matrix.len() {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
        if flagM {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
        if flagN {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use std::process::Output;

    use crate::problem_73::{self};

    #[test]
    fn test1() {
        // let mut input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        // let output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        // let mut input = vec![vec![1, 0]];
        // let output = vec![vec![0, 0]];
        let mut input = vec![vec![1, 0, 3]];
        let output = vec![vec![0, 0, 0]];
        problem_73::Solution::set_zeroes(&mut input);
        assert_eq!(input, output);
    }
}
