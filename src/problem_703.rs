use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k {
                heap.pop();
            }
        }
        KthLargest { heap, k }
    }

    fn kth(&self) -> i32 {
        self.heap.peek().unwrap().0
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k || val > self.kth() {
            self.heap.push(Reverse(val));
            if self.heap.len() > self.k {
                self.heap.pop();
            }
        }
        self.kth()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_703::{self};

    #[test]
    fn test1() {
        let mut kth = problem_703::KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
