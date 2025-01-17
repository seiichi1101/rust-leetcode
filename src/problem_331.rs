impl Solution {
    pub fn serialize(nodes: &mut Vec<&str>) -> bool {
        if let Some(node) = nodes.pop() {
            if node == "#" {
                true
            } else {
                Self::serialize(nodes) && Self::serialize(nodes)
            }
        } else {
            false
        }
    }
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut nodes: Vec<&str> = preorder.split(',').collect();
        nodes.reverse();
        Self::serialize(&mut nodes) && nodes.is_empty()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_331::{self};

    #[test]
    fn test1() {
        // assert_eq!(
        //     problem_331::Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()),
        //     true
        // );
        // assert_eq!(
        //     problem_331::Solution::is_valid_serialization("1,#".to_string()),
        //     false
        // );
        assert_eq!(
            problem_331::Solution::is_valid_serialization("9,#,#,1".to_string()),
            false
        );
    }
}
