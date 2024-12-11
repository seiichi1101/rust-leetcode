// brute force
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for i in 0..nums.len() {
//         for j in i + 1..nums.len() {
//             if nums[i] + nums[j] == target {
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }
//     vec![]
// }

// hash map
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match m.get(&(target - *v)) {
            Some(&j) => return vec![i as i32, j],
            None => m.insert(*v, i as i32),
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![2, 7, 1, 15];
        let targe = 9;
        assert_eq!(two_sum(input, targe), vec![0, 1]);
    }

    #[test]
    fn test2() {
        let input = vec![3, 2, 4];
        let targe = 6;
        assert_eq!(two_sum(input, targe), vec![1, 2]);
    }
}
