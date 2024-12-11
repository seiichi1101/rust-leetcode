impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(last) = stack.pop() {
                if "([{".contains(c) {
                    stack.push(last);
                    stack.push(c);
                } else if c == ')' && last != '(' {
                    return false;
                } else if c == ']' && last != '[' {
                    return false;
                } else if c == '}' && last != '{' {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_20::{self, Solution};

    #[test]
    fn test1() {
        // let input = "()";
        // let input = "()[]{}";
        // let input = "{[]}";
        let input = "[";
        // let input = "(]";
        assert_eq!(problem_20::Solution::is_valid(input.into()), false);
    }
}
