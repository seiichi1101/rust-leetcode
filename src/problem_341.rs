#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    list: Vec<i32>,
    index: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn retrieve(nestedList: Vec<NestedInteger>, res: &mut Vec<i32>) {
        for elem in nestedList {
            match elem {
                NestedInteger::List(list) => Self::retrieve(list, res),
                NestedInteger::Int(value) => res.push(value),
            }
        }
    }

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut tmp = vec![];
        Self::retrieve(nestedList, &mut tmp);
        NestedIterator {
            list: tmp,
            index: -1,
        }
    }

    fn next(&mut self) -> i32 {
        self.index += 1;
        self.list[self.index as usize]
    }

    fn has_next(&self) -> bool {
        self.list.get((self.index + 1) as usize).is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_341::{self};

    use super::NestedInteger;

    #[test]
    fn test1() {
        let input = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        let mut res = problem_341::NestedIterator::new(input);
        println!("{:?}", res.list);
        println!("{:?}", res.next());
        println!("{:?}", res.next());
        println!("{:?}", res.next());
        println!("{:?}", res.next());
        println!("{:?}", res.next());
    }
}
