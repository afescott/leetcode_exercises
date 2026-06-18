use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sherlockAndAnagrams' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn sherlockAndAnagrams(s: &str) -> i32 {
    let mut char_duplicates: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut vec = Vec::new();
    for i in 0..s.len() {
        for j in (i + 1)..=s.len() {
            /*             let sub = &s[i..j]; */
            // ...
            // s[i..j] is a substring

            vec.push(s[i..j].as_bytes());

            let mut val = s[i..j].as_bytes().to_vec();
            val.sort();

            *char_duplicates.entry(val).or_insert(0) += 1;
        }
    }

    println!("{:?}", char_duplicates);

    /* let duplicate_count = char_duplicates
    .values()
    .filter(|&&e| e > 1)
    .collect::<Vec<usize>>(); */

    char_duplicates
        .values()
        .map(|&n| {
            let val = (n * (n - 1) / 2) as i32;
            println!("{:?}", val);
            val
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlockAndAnagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
