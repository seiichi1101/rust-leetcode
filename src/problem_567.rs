use std::collections::HashMap;

impl Solution {
    // TLE
    // fn permute(prefix: String, s: String, permutations: &mut Vec<String>) {
    //     let n = s.len();
    //     if n == 0 {
    //         permutations.push(prefix);
    //     } else {
    //         for i in 0..n {
    //             let mut new_prefix = prefix.clone();
    //             let mut new_s = s.clone();
    //             new_prefix.push(s.chars().nth(i).unwrap());
    //             new_s.remove(i);
    //             Self::permute(new_prefix, new_s, permutations);
    //         }
    //     }
    // }
    // pub fn check_inclusion(s1: String, s2: String) -> bool {
    //     let mut permutations = Vec::new();
    //     Self::permute(String::new(), s1, &mut permutations);
    //     println!("permutations {:?}", permutations);

    //     for perm in permutations {
    //         if s2.contains(&perm) {
    //             return true;
    //         }
    //     }
    //     false
    // }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        for mut i in 0..(s2.len() - s1.len() + 1) {
            let mut sliced_s2 = s2.clone()[i..(i + s1.len())].to_string();

            let mut count = 0;
            for c in s1.chars() {
                count += 1;
                let index = sliced_s2.find(c);
                if let Some(idx) = index {
                    sliced_s2.remove(idx);
                } else {
                    break;
                }
            }
            if sliced_s2.is_empty() {
                return true;
            }
            i += count;
        }
        false
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_567::{self};
    #[test]
    fn test1() {
        let s1 = String::from("adc");
        let s2 = String::from("dcda");
        assert_eq!(problem_567::Solution::check_inclusion(s1, s2), true);
    }
}
