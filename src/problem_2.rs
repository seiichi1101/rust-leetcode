use crate::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ans = Some(Box::new(ListNode { val: 0, next: None }));
        let mut current_head = ans.as_mut();

        let mut carry = 0;
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        while l1.is_some() || l2.is_some() {
            let mut sum = 0;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_ref();
            }
            sum += carry;
            carry = sum / 10;
            sum %= 10;
            current_head.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: sum,
                next: None,
            }));
            current_head = current_head.unwrap().next.as_mut();
        }

        if carry != 0 {
            current_head.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: carry,
                next: None,
            }));
        }

        ans.unwrap().next
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_2::{self, ListNode};

    #[test]
    fn test1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let ans = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));
        assert_eq!(problem_2::Solution::add_two_numbers(l1, l2), ans);
    }
}
