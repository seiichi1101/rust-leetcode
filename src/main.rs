use core::num;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

mod problem_1;
mod problem_100;
mod problem_101;
mod problem_102;
mod problem_103;
mod problem_104;
mod problem_106;
mod problem_107;
mod problem_108;
mod problem_109;
mod problem_110;
mod problem_111;
mod problem_112;
mod problem_113;
mod problem_114;
mod problem_116;
mod problem_118;
mod problem_121;
mod problem_124;
mod problem_129;
mod problem_13;
mod problem_136;
mod problem_1382;
mod problem_14;
mod problem_144;
mod problem_145;
mod problem_146;
mod problem_15;
mod problem_151;
mod problem_156;
mod problem_169;
mod problem_2;
mod problem_20;
mod problem_206;
mod problem_207;
mod problem_21;
mod problem_222;
mod problem_226;
mod problem_230;
mod problem_234;
mod problem_235;
mod problem_250;
mod problem_255;
mod problem_257;
mod problem_268;
mod problem_270;
mod problem_283;
mod problem_285;
mod problem_314;
mod problem_35;
mod problem_404;
mod problem_501;
mod problem_54;
mod problem_543;
mod problem_563;
mod problem_567;
mod problem_572;
mod problem_617;
mod problem_637;
mod problem_653;
mod problem_671;
mod problem_7;
mod problem_70;
mod problem_700;
mod problem_704;
mod problem_72;
mod problem_73;
mod problem_74;
mod problem_76;
mod problem_783;
mod problem_938;
mod problem_94;
mod problem_95;
mod problem_96;
mod problem_965;
mod problem_98;
mod problem_99;
mod problem_tmp;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0)); // 初期値を持つrootノードをボックス化
        let mut current = root.as_mut(); // currentは可変の参照としてrootを指す

        for num in vec {
            let node = Box::new(ListNode::new(num)); // 新しいノードをボックス化
            current.next = Some(node); // currentのnextにnodeを割り当て
            current = current.next.as_mut().unwrap(); // currentを更新して新しいノードを指す
        }
        root.next // 最初のダミーノードをスキップして、実際のリストを返す
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn from_vec(i: usize, vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let val = vec.get(i);
        if let Some(Some(v)) = val {
            let mut node = Self::new(*v);
            let left = Self::from_vec(i * 2 + 1, vec.clone());
            let right = Self::from_vec(i * 2 + 2, vec.clone());

            node.left = left;
            node.right = right;
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        main();
    }
}
