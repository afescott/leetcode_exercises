use std::{
    collections::{BTreeMap, HashMap},
    io::{self, BufRead},
};

/*
 * Complete the 'checkMagazine' function below.
 *
 * The function accepts following parameters:
 *  1. STRING_ARRAY magazine
 *  2. STRING_ARRAY note
 */

fn checkMagazine(magazine: &[String], note: &[String]) {
    let mut magazine_hashmap: HashMap<String, usize> = HashMap::new();

    for ele in magazine.iter().enumerate() {
        magazine_hashmap
            .entry(ele.1.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let mut invalidated = false;
    for ele in note.iter().enumerate() {
        let res = magazine_hashmap.get_mut(ele.1);
        if let Some(val) = res {
            if val < &mut 1 {
                invalidated = true;
            }
            *val -= 1;
        } else {
            invalidated = true;
        }
    }

    if invalidated {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let n = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let magazine: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let note: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    checkMagazine(&magazine, &note);
}
