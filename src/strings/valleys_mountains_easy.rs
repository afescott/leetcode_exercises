use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

// UDDDUDUU

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut elevation = 0;
    let mut valley = 0;
    let mut mount_fin;
    // gives you raw u8s with no decoding
    for ele in path.bytes() {
        mount_fin = false;
        if ele == b'D' {
            elevation += 1;
        } else {
            mount_fin = true;
            elevation -= 1;
        }
        if elevation == 0 && mount_fin {
            valley += 1;
        }
    }

    valley
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
