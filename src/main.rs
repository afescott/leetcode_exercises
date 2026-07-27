use std::{collections::HashMap, fmt::Debug};

pub fn number_to_words(num: i32) -> String {
    let one_to_twenty = Vec::from([
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
    ]);
    let mut tens = Vec::from([
        "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ]);

    let mut hundreds = Vec::from([
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

    let thousands = ["Hundred", "Thousand", "Million", "Billion"];

    let num = match num {
        0..19 => one_to_twenty[num as usize].to_string(),
        20..99 => {
            let tens_place = num / 10;
            let value = tens_place % tens_place;

            let result = tens.get((tens_place - 1) as usize).unwrap();

            let one_to_twenty = one_to_twenty[value as usize];

            let str = format!("{:?} {:?}", result, one_to_twenty);
            str
        }
        100..999 => {
            let hundreds_place = num / 100;

            let hundreds = hundreds.get((hundreds_place - 1) as usize).unwrap();

            let tens_place = (num / 10) / 10;
            let value = tens_place % tens_place;

            println!("{:?}", value);
            let result = tens.get((tens_place - 1) as usize).unwrap();

            println!("{:?}", result);
            let tens = one_to_twenty[value as usize];

            let last_ele = num.to_string().chars().nth(2).unwrap();
            let comeon: usize = last_ele.to_digit(10).unwrap() as usize;

            let word = one_to_twenty.get(comeon).copied();

            let str = format!("{:?} {:?} {:?}", hundreds, result, tens);
            str
        }
        /* 1000..9999 => println!(),
        10000..99999 => println!(),
        100000..999999 => println!(), */
        _ => String::new(),
    };

    num
}

fn main() {}
