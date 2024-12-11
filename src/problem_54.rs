use std::{collections::HashMap, iter::Map};

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];

    let x_max = matrix.len() + 1;
    let y_max = matrix[0].len() + 1;
    #[derive(Debug)]
    enum Arrow {
        Right,
        Down,
        Left,
        Up,
    }
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct Point {
        x: usize,
        y: usize,
    }
    #[derive(Debug)]
    struct Result {
        arrow: Arrow,
        map: HashMap<Point, bool>,
    }
    let mut route = Result {
        arrow: Arrow::Right,
        map: HashMap::new(),
    };
    for i in 1..x_max {
        for j in 1..y_max {
            route.map.insert(Point { x: i, y: j }, true);
        }
    }

    let mut next = Point { x: 1, y: 1 };
    // while let Some(visit) = route.map.get(&next) {}
    while *route.map.get(&next).unwrap_or(&false) {
        if let Arrow::Right = route.arrow {
            while next.x < x_max && next.y < y_max && *route.map.get(&next).unwrap_or(&false) {
                res.push(matrix[next.x - 1][next.y - 1]);
                route.map.insert(next.clone(), false);
                next.y += 1
            }
            next.y -= 1;
            route.arrow = Arrow::Down;
            next.x += 1;
        }
        if let Arrow::Down = route.arrow {
            while next.x < x_max && next.y < y_max && *route.map.get(&next).unwrap_or(&false) {
                res.push(matrix[next.x - 1][next.y - 1]);
                route.map.insert(next.clone(), false);
                next.x += 1
            }
            next.x -= 1;
            route.arrow = Arrow::Left;
            next.y -= 1;
        }
        if let Arrow::Left = route.arrow {
            while next.x < x_max && next.y < y_max && *route.map.get(&next).unwrap_or(&false) {
                res.push(matrix[next.x - 1][next.y - 1]);
                route.map.insert(next.clone(), false);
                next.y -= 1
            }
            next.y += 1;
            route.arrow = Arrow::Up;
            next.x -= 1;
        }
        if let Arrow::Up = route.arrow {
            while next.x < x_max && next.y < y_max && *route.map.get(&next).unwrap_or(&false) {
                res.push(matrix[next.x - 1][next.y - 1]);
                route.map.insert(next.clone(), false);
                next.x -= 1
            }
            next.x += 1;
            route.arrow = Arrow::Right;
            next.y += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(input), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test2() {
        let input = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            spiral_order(input),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
