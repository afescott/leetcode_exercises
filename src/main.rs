impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack == needle {
            return 0;
        }

        for i in 0..haystack.len() {
            let split = &haystack[i..haystack.len()];
            let mut word = String::new();
            let mut chars = needle.chars();
            for ele in split.chars() {
                let val = chars.next();

                if val.is_some_and(|ch| ch == ele) {
                    word.push(ele);
                    if word == needle {
                        return i.try_into().unwrap();
                    }
                } else {
                    break;
                }
            }
        }
        -1
    }
}

fn main() {}
