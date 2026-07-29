struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        // index 0 is "" so digit 1 → "One", not "Zero"
        let one_to_twenty = Vec::from([
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ]);
        let tens = Vec::from([
            "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ]);

        let hundreds = Vec::from([
            "One Hundred",
            "Two Hundred",
            "Three Hundred",
            "Four Hundred",
            "Five Hundred",
            "Six Hundred",
            "Seven Hundred",
            "Eight Hundred",
            "Nine Hundred",
        ]);

        let num = match num {
            1..=19 => one_to_twenty[num as usize].to_string(),
            20..=99 => {
                let tens_place = num / 10;
                let ones_place = num % 10;

                let result = tens.get((tens_place - 1) as usize).unwrap();

                if ones_place == 0 {
                    result.to_string()
                } else {
                    let ones = one_to_twenty[ones_place as usize];
                    format!("{} {}", result, ones)
                }
            }
            100..=999 => {
                let hundreds_place = num / 100;
                let hundreds = hundreds.get((hundreds_place - 1) as usize).unwrap();

                let rest = num % 100;

                if rest == 0 {
                    hundreds.to_string()
                } else if rest < 20 {
                    format!("{} {}", hundreds, one_to_twenty[rest as usize])
                } else {
                    let tens_place = rest / 10;
                    let ones_place = rest % 10;

                    let result = tens.get((tens_place - 1) as usize).unwrap();

                    if ones_place == 0 {
                        format!("{} {}", hundreds, result)
                    } else {
                        let ones = one_to_twenty[ones_place as usize];
                        format!("{} {} {}", hundreds, result, ones)
                    }
                }
            }
            1000..=9999 => {
                let thousands_place = num / 1000;
                let thousands = one_to_twenty[thousands_place as usize];

                let rest = num % 1000;
                if rest == 0 {
                    format!("{} Thousand", thousands)
                } else {
                    format!("{} Thousand {}", thousands, Self::number_to_words(rest))
                }
            }
            10000..=99999 => {
                let tens_of_thousands = num / 1000;
                let thousands_word = Self::number_to_words(tens_of_thousands);

                let rest = num % 1000;
                if rest == 0 {
                    format!("{} Thousand", thousands_word)
                } else {
                    format!(
                        "{} Thousand {}",
                        thousands_word,
                        Self::number_to_words(rest)
                    )
                }
            }
            100000..=999999 => {
                let hundred_thousands = num / 1000;
                let thousands_word = Self::number_to_words(hundred_thousands);

                let rest = num % 1000;
                if rest == 0 {
                    format!("{} Thousand", thousands_word)
                } else {
                    format!(
                        "{} Thousand {}",
                        thousands_word,
                        Self::number_to_words(rest)
                    )
                }
            }
            1_000_000..=999_999_999 => {
                let millions = num / 1_000_000;
                let millions_word = Self::number_to_words(millions);

                let rest = num % 1_000_000;
                if rest == 0 {
                    format!("{} Million", millions_word)
                } else {
                    format!("{} Million {}", millions_word, Self::number_to_words(rest))
                }
            }
            1_000_000_000..=i32::MAX => {
                let billions = num / 1_000_000_000;
                let billions_word = one_to_twenty[billions as usize];

                let rest = num % 1_000_000_000;
                if rest == 0 {
                    format!("{} Billion", billions_word)
                } else {
                    format!("{} Billion {}", billions_word, Self::number_to_words(rest))
                }
            }
            _ => String::new(),
        };

        num
    }
}
