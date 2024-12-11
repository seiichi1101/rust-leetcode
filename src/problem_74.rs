impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() > 1 {
            let mid_x = matrix.len() / 2;
            if target < matrix[mid_x][0] {
                Self::search_matrix(matrix[..mid_x].to_vec(), target)
            } else {
                Self::search_matrix(matrix[mid_x..].to_vec(), target)
            }
        } else if matrix[0].len() > 1 {
            let mid_y = matrix[0].len() / 2;
            if target < matrix[0][mid_y] {
                Self::search_matrix(vec![matrix[0][..mid_y].to_vec()], target)
            } else {
                Self::search_matrix(vec![matrix[0][mid_y..].to_vec()], target)
            }
        } else {
            target == matrix[0][0]
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_74::{self};

    #[test]
    fn test1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert_eq!(problem_74::Solution::search_matrix(matrix, target), true);
    }
    #[test]
    fn test2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 60;
        assert_eq!(problem_74::Solution::search_matrix(matrix, target), true);
    }
    #[test]
    fn test3() {
        let matrix = vec![vec![1], vec![3]];
        let target = 4;
        assert_eq!(problem_74::Solution::search_matrix(matrix, target), false);
    }
}
