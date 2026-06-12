use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'makeAnagram' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING a
 *  2. STRING b
 */

fn makeAnagram(a: &str, b: &str) -> i32 {
    let mut hashmap: HashMap<char, i32> = HashMap::new();

    for ele in a.chars() {
        *hashmap.entry(ele).or_insert(0) += 1;
    }
    for ele in b.chars() {
        *hashmap.entry(ele).or_insert(0) -= 1; // <- entry, not indexing
    }

    hashmap.values().map(|v| v.abs()).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a = stdin_iterator.next().unwrap().unwrap();

    let b = stdin_iterator.next().unwrap().unwrap();

    let res = makeAnagram(&a, &b);

    writeln!(&mut fptr, "{}", res).ok();
}
