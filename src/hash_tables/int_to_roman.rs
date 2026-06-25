impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let pairs = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut romanised_int = String::new();
        let mut number = num;
        for (key, value) in pairs {
            let div_result = number / value;

            if div_result > 0 {
                for _ in 0..div_result {
                    romanised_int.push_str(key);
                }
                number = number - (div_result * value);
                println!("{}: {} - {}", key, div_result, number);
            }
        }

        romanised_int
    }
}
