use std::sync::{Arc, Condvar, Mutex};

struct ZeroEvenOdd {
    n: i32,
    current_number: Arc<Mutex<i32>>,
    cond_var: Arc<(Mutex<i32>, Condvar)>,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            current_number: Arc::new(Mutex::new(0)),
            cond_var: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }

    fn verify_number(&self, cur_num: i32) -> bool {
        let usize = self.n as usize;
        if cur_num.to_string().len() == usize * 2 {
            return false;
        }

        true
    }

    //if current_thread number length = 2 x n ignore

    // printNumber(x) prints the integer x
    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let (value, cond_var) = &*self.cond_var;

        let mut res = value.lock().unwrap();

        if *res == 5 {
            *res = 0;

            let mut number = self.current_number.lock().unwrap();

            if self.verify_number(*number) {
                let number_clone = number.to_string();

                match number_clone.chars().last() {
                    Some('1') => {
                        *number = *number * 10 + 1;
                    }
                    Some('2') => {
                        *number = *number * 10 + 2;
                    }
                    Some('3') => {
                        *number = *number * 10 + 3;
                    }
                    Some('4') => {
                        *number = *number * 10 + 4;
                    }
                    Some('5') => {
                        *number = *number * 10 + 5;
                    }
                    _ => return,
                }
            }
        }

        cond_var.notify_all();
        print_number(*res);
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let (value, cond_var) = &*self.cond_var;

        let mut res = value.lock().unwrap();

        if *res == 1 {
            *res = 2;

            let mut number = self.current_number.lock().unwrap();
            if self.verify_number(*number) {
                *number = *number * 10 + 2;
            }
        }

        cond_var.notify_all();
        print_number(*res);
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let (value, cond_var) = &*self.cond_var;

        let mut res = value.lock().unwrap();

        if *res == 0 {
            *res = 1;

            let mut number = self.current_number.lock().unwrap();
            if self.verify_number(*number) {
                *number = *number * 10 + 1;
            }
        }

        cond_var.notify_all();
        print_number(*res);
    }
}

fn main() {}
