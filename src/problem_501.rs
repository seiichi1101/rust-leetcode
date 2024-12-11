use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::TreeNode;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut map = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            if let Some(elem) = queue.pop_front().unwrap() {
                let key = elem.borrow().val;
                if map.contains_key(&key) {
                    if let Some(current_val) = map.get(&key) {
                        map.insert(key, current_val + 1);
                    } else {
                        map.insert(key, 1);
                    }
                } else {
                    map.insert(key, 1);
                }

                queue.push_back(elem.borrow().left.clone());
                queue.push_back(elem.borrow().right.clone());
            }
        }

        let mut items: Vec<_> = map.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1));

        let mut max_value = items[0].1;
        for (key, value) in items {
            if value != max_value {
                break;
            }
            res.push(*key);
        }

        res
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_501, TreeNode};

    #[test]
    fn test1() {
        let input = TreeNode::from_vec(0, vec![Some(1), None, Some(2), None, None, Some(2)]);
        assert_eq!(problem_501::Solution::find_mode(input), vec![2]);
    }
}
