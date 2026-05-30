/*
 * Complete the 'hourglassSum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

//attempt #2
fn hourglassSum(arr: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for i in 0..=3 {
        println!("{:?}", i);
        for j in 0..=3 {
            let row_1 = arr[i][j] + arr[i][j + 1] + arr[i][j + 2];
            let row_2 = arr[i + 1][j + 1];
            let row_3 = arr[i + 2][j] + arr[i + 2][j + 1] + arr[i + 2][j + 2];

            let temp_sum = row_1 + row_2 + row_3;

            if temp_sum > sum {
                sum = temp_sum;
            }
        }
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(6_usize);

    for i in 0..6_usize {
        arr.push(Vec::with_capacity(6_usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = hourglassSum(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

// Attempt 1
fn hourglassSum(arr: &[Vec<i32>]) -> i32 {
    let mut highest_value = 0;

    let mut row_index = 0;
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for row in arr {
        //new hourglass
        if row_index == 2 {
            if sum_1 > sum_2 {
                highest_value = sum_1;
            } else {
                highest_value = sum_2;
            }
            sum_1 = 0;
            sum_2 = 0;
        }
        for col_index in 0..row.len() {
            //different hourglass
            //

            if col_index > 2 {
                sum_2 += row[col_index];
            } else {
                sum_1 += row[col_index];
            }
        }
        row_index += 1;
    }
    highest_value
}
