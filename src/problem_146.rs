use std::{cell::RefCell, collections::HashMap, env::current_exe, rc::Rc};

struct DoublyLinkedList {
    key: i32,
    prev: Option<Rc<RefCell<DoublyLinkedList>>>,
    next: Option<Rc<RefCell<DoublyLinkedList>>>,
}
struct LRUCache {
    map: HashMap<i32, i32>,
    size: i32,
    head: Option<Rc<RefCell<DoublyLinkedList>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            head: None,
            size: capacity,
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(val) = self.map.get(&key) {
            // linked listの先頭に移動
            *val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(val) = self.map.get(&key) {
            // linked listの先頭に移動
            // let mut current = self.head.clone();
            // while current.is_some() {
            //     if key == current.unwrap().borrow().key {
            //         let prev_node = current.unwrap().borrow().prev;
            //         let next_node = current.unwrap().borrow().next;
            //         match (prev_node, next_node) {
            //             (None, None) => todo!(),
            //             (None, Some(_)) => todo!(),
            //             (Some(_), None) => todo!(),
            //             (Some(_), Some(_)) => todo!(),
            //         }
            //         current.unwrap().borrow_mut().prev = None;
            //         current.unwrap().borrow_mut().next = self.head.clone();
            //         self.head.unwrap().borrow_mut().prev = current;
            //         break;
            //     }
            // }
        } else {
            // linked listの先頭に追加 & size超えてれば末尾削除
            let mut new_linked_list = Rc::new(RefCell::new(DoublyLinkedList {
                key,
                prev: None,
                next: None,
            }));
            if let Some(h) = self.head.clone() {
                new_linked_list.borrow_mut().next = Some(h.clone());
                h.borrow_mut().prev = Some(new_linked_list.clone());
                self.head = Some(new_linked_list.clone());

                let mut current = self.head.clone();
                let mut count = 0;
                while current.is_some() {
                    if count == self.size {
                        self.map.remove(
                            &current
                                .clone()
                                .unwrap()
                                .borrow_mut()
                                .next
                                .clone()
                                .unwrap()
                                .borrow()
                                .key,
                        );
                        current.unwrap().borrow_mut().next = None;
                        break;
                    }
                    current = current.unwrap().borrow().next.clone();
                    count += 1;
                }
            } else {
                self.head = Some(new_linked_list.clone());
            }
            self.map.insert(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_146::{self};

    #[test]
    fn test1() {
        let mut lru = problem_146::LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(lru.get(2), -1); // returns -1 (not found)
        lru.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(lru.get(1), -1); // return -1 (not found)
        assert_eq!(lru.get(3), 3); // return 3
        assert_eq!(lru.get(4), 4); // return 4
    }
}
