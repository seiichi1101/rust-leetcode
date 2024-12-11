use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // HashMapにしたほうが計算効率がいい
        let mut dependencies = vec![vec![]; num_courses as usize];
        for i in 0..prerequisites.len() {
            dependencies
                .get_mut(prerequisites[i][0] as usize)
                .unwrap()
                .push(prerequisites[i][1] as usize);
        }

        let mut queue = VecDeque::new();
        for empty_node in
            dependencies
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if x.is_empty() { Some(i) } else { None })
        {
            queue.push_back(empty_node);
        }

        let mut count = 0;
        let mut ordered = vec![];
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            ordered.push(node);

            dependencies
                .iter_mut()
                .for_each(|inner| inner.retain(|&x| x != node));

            if queue.is_empty() {
                for empty_node in dependencies.iter().enumerate().filter_map(|(i, x)| {
                    if x.is_empty() && !ordered.contains(&i) {
                        Some(i)
                    } else {
                        None
                    }
                }) {
                    queue.push_back(empty_node);
                }
            }
            count += 1;
        }

        count == num_courses
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_207::{self};

    #[test]
    fn test1() {
        let num_courses = 2;
        let prerequisites = vec![];
        // let prerequisites = vec![vec![1, 0]];
        // let prerequisites = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(
            problem_207::Solution::can_finish(num_courses, prerequisites),
            true
        );
    }
}
