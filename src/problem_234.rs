use crate::ListNode;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut list = vec![];
        let mut current = head;
        while let Some(c) = current {
            list.push(c.val);
            current = c.next;
        }

        let mut l: usize = 0;
        let mut r: usize = list.len() - 1;

        while l <= r {
            if list[l] != list[r] {
                return false;
            }
            l += 1;
            r = match r.checked_sub(1) {
                Some(new_r) => new_r,
                None => break,
            };
        }
        true
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_234::{self, ListNode};

    #[test]
    fn test1() {
        let input = ListNode::from_vec(vec![1, 2, 2, 1]);
        // let input = ListNode::from_vec(vec![1]);
        let output = true;
        assert_eq!(problem_234::Solution::is_palindrome(input), output);
    }
}
