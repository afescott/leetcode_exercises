use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'jumpingOnClouds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY c as parameter.
 */

fn jumpingOnClouds(c: &[i32]) -> i32 {
    let mut jump_counter = 0;
    let mut skip = 0;
    for i in 0..c.len() - 1 {
        if c[i + 1] == 0 && c[i] == 0 {
            skip += 1;
            if skip == 2 {
                jump_counter += 1;
                skip = 0;
            }
        } else {
            skip = 0;
            jump_counter += 1;
        }
    }

    jump_counter
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let c: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumpingOnClouds(&c);

    writeln!(&mut fptr, "{}", result).ok();
}
