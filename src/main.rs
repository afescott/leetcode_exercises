use std::{collections::HashMap, fmt::Debug};

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let words = [
            "Zero",
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
        ];
        let tens = [
            "Ten", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        let val = HashMap::from([
            (10, "Ten"),
            (20, "Twenty"),
            (30, "Thirty"),
            (40, "Forty"),
            (50, "Fifty"),
            (60, "Sixty"),
            (70, "Seventy"),
            (80, "Eighty"),
            (90, "Ninety"),
        ]);
        let hash_map = HashMap::from((10, "Ten"));
        let thousands = ["Hundred", "Thousand", "Million", "Billion"];

        let num = match num {
            0..19 => words[num as usize],
            20..99 => {
                let tens_place = num / 10;
                let value = tens_place * 10;

                let result = val.entry(value);
            }
            /* 100..999 => println!(),
            1000..9999 => println!(),
            10000..99999 => println!(),
            100000..999999 => println!(), */
            _ => &String::new(),
        };

        String::new()
    }
}

fn main() {}
