pub fn climb_stairs(n: i32) -> i32 {
    let mut prev_step = 2;
    let mut prev_prev_step = 1;

    if n == 1 {
        return prev_prev_step;
    }
    if n == 2 {
        return prev_step;
    }
    for _ in 0..n - 1 {
        let next = prev_step + prev_prev_step;

        prev_prev_step = prev_step;

        prev_step = next;
    }

    println!("{:?}, {:?}", prev_step, prev_prev_step);
    prev_prev_step
}

fn main() {}
