use std::cell::RefCell;
use std::rc::Rc;

use crate::{ListNode, TreeNode};
impl Solution {
    pub fn find_size(head: Option<Box<ListNode>>) -> i32 {
        let mut count = 0;
        let mut root = head;
        while let Some(r) = root {
            root = r.next;
            count += 1;
        }
        count
    }
    pub fn convert(
        mut head: &mut Option<Box<ListNode>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r || head.is_none() {
            return None;
        }
        let mid = (l + r) / 2;

        let left = Self::convert(head, l, mid - 1);
        let val = head.as_ref().unwrap().val;

        *head = head.as_mut().unwrap().next.take();

        let right = Self::convert(head, mid + 1, r);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let size = Self::find_size(head.clone());
        Self::convert(&mut head.clone(), 0, size - 1)
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_109, ListNode, TreeNode};

    #[test]
    fn test1() {
        let input = ListNode::from_vec(vec![-10, -3, 0, 5, 9]);
        let ans = TreeNode::from_vec(
            0,
            vec![Some(0), Some(-10), Some(5), None, Some(-3), None, Some(9)],
        );
        assert_eq!(problem_109::Solution::sorted_list_to_bst(input), ans)
    }
}
