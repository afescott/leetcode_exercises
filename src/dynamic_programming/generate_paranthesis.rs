impl Solution {
    fn backtrack(result: &mut Vec<String>, current: &mut String, open: i32, closed: i32, n: i32) {
        if current.len() == (2 * n) as usize {
            result.push(current.clone());
            return;
        }
        if open < n {
            current.push('(');
            backtrack(result, current, open + 1, closed, n);
            current.pop();
        }
        if closed < open {
            current.push(')');
            backtrack(result, current, open, closed + 1, n);
            current.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        backtrack(&mut result, &mut current, 0, 0, n);

        if current.len() == (2 * n) as usize {
            result.push(current.clone());
            return;
        }
        if open < n {
            current.push('(');
            backtrack(result, current, open + 1, closed, n);
            current.pop();
        }
        if closed < open {
            current.push(')');
            backtrack(result, current, open, closed + 1, n);
            current.pop();
        }

        /* //attempt no #1
        let mut open = 0;
        let mut closed = 0;
        let mut string = String::new();
        let mut vec_of_paranthesis = Vec::new();
        for i in 0..2 * n {
            if string.len() < (2 * n) as usize {
                if open <= closed {
                    string.push('(');
                } else {
                    string.push(')');
                }
            } else {
                vec_of_paranthesis.push(string);

                string = String::new();
            }
        }
        let val_2 = 2 % n; */
    }
}
