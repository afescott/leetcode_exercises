impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack == needle {
            return 0;
        }

        if needle.len() > haystack.len() {
            return -1;
        }

        //we only need to iterate h-n times otherwise the length of needle won't fit in latter end
        //of haystack
        for i in 0..=haystack.len() - needle.len() {
            // needle = ll haystack = hello. 'he', 'el', 'll'
            if &haystack[i..i + needle.len()] == needle {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {}
