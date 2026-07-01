use std::{
    sync::{atomic::Ordering, Arc, Condvar, Mutex},
    thread::sleep,
    time::Duration,
};

struct Foo {
    atom: std::sync::atomic::AtomicUsize,
}

impl Foo {
    fn new() -> Self {
        Foo {
            atom: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // Do not change this line

        print_first();

        self.atom.store(1, Ordering::SeqCst);

        /* let mut started = self.cond.0.lock().unwrap();

        *started = true; */
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        while self.atom.load(Ordering::SeqCst) < 1 {}

        // Do not change this line
        print_second();
        self.atom.store(2, Ordering::SeqCst);
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        while self.atom.load(Ordering::SeqCst) < 2 {}
        // Do not change this line
        print_third();
    }
}
