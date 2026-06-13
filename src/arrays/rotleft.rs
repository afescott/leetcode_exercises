use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'rotLeft' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER d
 */

fn rotLeft(a: &[i32], d: i32) -> Vec<i32> {
    //Attempt #1
    /* let mut final_vec = a.to_vec();

    let mut temp_vec = a.to_vec();
    for _ in 0..d {
        println!("{:?}", temp_vec.len());
        for ele_1 in 0..a.len() - 1 {
            if ele_1 == 0 {
                final_vec[a.len() - 1] = temp_vec[ele_1];
                final_vec[ele_1] = temp_vec[ele_1 + 1];
            } else {
                final_vec[ele_1] = temp_vec[ele_1 + 1];
            }
            println!("{:?}", temp_vec);
        }
        temp_vec = final_vec.clone();
    }

    final_vec */

    //Attempt #2

    // 4 % 5 = 4
    let d = (d as usize) % a.len();

    println!("{:?}", d);

    let vals = &a;

    println!("{:?}", vals);

    // a[4..] = [1, 2, 3, 4, 5]
    let vals = &a[d..];

    // a[4..] = [5]
    let vals_2 = &a[..d];

    println!("{:?}", vals);
    println!("{:?}", vals_2);

    a[d..].iter().chain(a[..d].iter()).copied().collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let d = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = rotLeft(&a, d);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
