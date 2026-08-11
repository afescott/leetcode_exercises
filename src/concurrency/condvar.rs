use std::sync::{Arc, Condvar, Mutex};

struct FooBar {
    n: usize,
    cond_var: Arc<(Mutex<bool>, Condvar)>,
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar {
            n,
            cond_var: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();

            let res = &*self.cond_var;
            *res.0.lock().unwrap() = true;
            res.1.notify_one();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            // printBar() outputs "bar". Do not change or remove this line.

            let (lock, cvar) = &*self.cond_var;
            let mut turn = lock.lock().unwrap();
            while !*turn {
                turn = cvar.wait(turn).unwrap();
            }

            *turn = false;
            print_bar();
        }
    }
}
