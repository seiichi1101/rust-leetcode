pub fn missing_number(nums: Vec<i32>) -> i32 {
    let sum = (1 + nums.len() as i32) * (nums.len() as i32) / 2;
    nums.iter().fold(sum, |acc, x| acc - x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![3, 0, 1];
        assert_eq!(missing_number(input), 2);
    }

    #[test]
    fn test2() {
        let input = vec![0, 1];
        assert_eq!(missing_number(input), 2)
    }

    #[test]
    fn test3() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(missing_number(input), 8)
    }

    #[test]
    fn test4() {
        let input = vec![1];
        assert_eq!(missing_number(input), 0)
    }
}
