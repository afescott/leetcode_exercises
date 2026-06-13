use std::io::{self, BufRead};

/*
 * Complete the 'minimumBribes' function below.
 *
 * The function accepts INTEGER_ARRAY q as parameter.
 */

fn minimumBribes(q: &[i32]) {
    //Functional attempt 1
    /* // 1 2 3 4 5
    // 2 1 5 3 4  = 3 bribes
    let mut is_choatic = false;
    let mut count = 0;

    for i in 0..q.len() - 1 {
        if q[i] > q[i + 1] {
            count += 1;
            for j in i + 1..q.len() - 1 {
                if q[i] > q[j + 1] {
                    count += 1;
                    if count > 2 {
                        is_choatic = true;
                    }
                } else {
                    break;
                }
            }
        }
    }

    if is_choatic {
        println!("Too chaotic");
    } else {
        println!("{}", count);
    } */

    //Attempt success 2
    let mut count = 0;
    let mut temp_count = 0;

    // total bribes
    for i in 0..q.len() {
        for j in i + 1..q.len() {
            if q[i] > q[j] {
                temp_count += 1;
                if temp_count > 2 {
                    println!("Too chaotic");
                    return;
                }
                count += 1;
            }
        }
        temp_count = 0;
    }

    println!("{}", count);
    /* // chaos check
    for i in 0..q.len() {
        if q[i] - 1 - i as i32 > 2 {
            is_choatic = true;
        }
    } */

    /* //Almost success attempt 3
    let mut total = 0;
    for i in 0..q.len() {
        let bribes = q[i] - 1 - i as i32;
        if bribes > 2 {
            println!("Too chaotic");
            return;
        }
        if bribes > 0 {
            total += bribes;
        }
    }
    println!("{}", total); */
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let q: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        minimumBribes(&q);
    }
}
