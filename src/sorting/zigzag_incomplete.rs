impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // |    /|    /|    /|
        // |  /  |  /  |  /  |
        // |/    |/    |/    |

        let mut column_set = Vec::new();
        for i in 0..num_rows {
            println!("{:?}", i / num_rows);
            let mut new_column = String::new();

            let mut value = num_rows;
            for i in 0..num_rows {
                let ele = s.get(0..(num_rows as usize));
            }
            /*             let character = s[num_rows + 2]; */

            /* for ele in s.chars() {
                if i - 1 == num_rows {
                    println!("{:?}", new_column);
                    column_set.push(new_column);
                    new_column = String::new();
                }
                new_column.push(ele);
            } */
        }
        String::new()
    }
}
