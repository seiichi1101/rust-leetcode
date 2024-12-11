use crate::ListNode;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_node = head;
        let mut previous_node = None;
        while let Some(mut c_node) = current_node {
            let next_node = c_node.next.take();
            c_node.next = previous_node;
            previous_node = Some(c_node);
            current_node = next_node;
        }
        previous_node
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_206::{self};
    use crate::ListNode;

    #[test]
    fn test1() {
        let input = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let output = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(problem_206::Solution::reverse_list(input), output);
    }
}
