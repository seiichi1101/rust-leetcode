use crate::ListNode;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));
        let mut current = root.as_mut();
        let mut l1 = list1.as_ref();
        let mut l2 = list2.as_ref();
        while l1.is_some() || l2.is_some() {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        current.next = l1.cloned();
                        l1 = l1.unwrap().next.as_ref();
                    } else {
                        current.next = l2.cloned();
                        l2 = l2.unwrap().next.as_ref();
                    }
                }
                (Some(_n1), None) => {
                    current.next = l1.cloned();
                    l1 = l1.unwrap().next.as_ref();
                }
                (None, Some(_n2)) => {
                    current.next = l2.cloned();
                    l2 = l2.unwrap().next.as_ref();
                }
                _ => {
                    break;
                }
            }
            current = current.next.as_mut().unwrap().as_mut();
        }
        root.next
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_21::{self, Solution},
        ListNode,
    };

    #[test]
    fn test1() {
        let input1 = ListNode::from_vec(vec![1, 2, 4]);
        let input2 = ListNode::from_vec(vec![1, 3, 4]);
        assert_eq!(
            problem_21::Solution::merge_two_lists(input1, input2),
            ListNode::from_vec(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
