use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        let mut current = root;
        while let Some(node) = current {
            let n = node.borrow();
            stack.push(Some(node.clone()));
            current = n.left.clone();
        }
        BSTIterator { stack }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap().unwrap();
        let n = node.borrow();
        let mut current = n.right.clone();
        while let Some(node) = current {
            let n = node.borrow();
            self.stack.push(Some(node.clone()));
            current = n.left.clone();
        }

        n.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        problem_173::{self},
        TreeNode,
    };

    #[test]
    fn test1() {
        let mut bst = problem_173::BSTIterator::new(TreeNode::from_vec(
            0,
            vec![Some(7), Some(3), Some(15), None, None, Some(9), Some(20)],
        ));
        assert_eq!(bst.next(), 3); // return 3
        assert_eq!(bst.next(), 7); // return 7
        assert_eq!(bst.has_next(), true); // return True
        assert_eq!(bst.next(), 9); // return 9
        assert_eq!(bst.has_next(), true); // return True
        assert_eq!(bst.next(), 15); // return 15
        assert_eq!(bst.has_next(), true); // return True
        assert_eq!(bst.next(), 20); // return 20
        assert_eq!(bst.has_next(), false); // return False
    }
}
