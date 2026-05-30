use std::{
    collections::VecDeque,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Condvar, Mutex,
    },
};

struct BoundedBlockingQueue {
    capacity: usize,
    queue: VecDeque<usize>,
    tx: Sender<usize>,
    rx: Receiver<usize>,
}

impl BoundedBlockingQueue {
    fn new(capacity: usize) -> Self {
        let (tx, rx) = channel();
        Self {
            capacity,
            queue: VecDeque::new(),
            tx,
            rx,
        }
    }

    fn enqueue(&self, ele: usize) {
        while self.queue.len() == self.capacity {
            self.condvar.wait();
            println!("Queue is full waiting for a slot to come available");
        }
    }

    fn start(&self) {
        let cond = Condvar::new();

        let value = Arc::new((Mutex::new(false), Condvar::new()));

        let (lock, cond) = &*value;

        let started = lock.lock().unwrap();

        while !*started {
            cond.wait(started).unwrap();
        }
    }
}

/* BoundedBlockingQueue(int capacity): The constructor initializes the queue with a maximum capacity.

void enqueue(int element): Adds an element to the front of the queue. If the queue is full, the calling thread must block (wait) until space becomes available.

int dequeue(): Removes and returns an element from the rear of the queue. If the queue is empty, the calling thread must block (wait) until an element becomes available.

int size():
*/

/* struct BoundedBlockingQueue {
    capacity: usize,
    queue: VecDeque<usize>,
    tx: Sender<usize>,
    rx: Receiver<usize>,
}

impl BoundedBlockingQueue {
    fn new(capacity: usize) -> Self {
        let (tx, rx) = channel();
        Self {
            capacity,
            queue: VecDeque::new(),
            tx,
            rx,
        }
    }

    fn start(&self) {
        let closure | = {
            if self.queue.len() == self.capacity - 1 {
                println!("Queue is full waiting for a slot to come available");
                false
            }
            true
        };
        while let Ok(rx) = self.rx.recv() {
            while !closure() {
            }
        }
    }

    fn enqueue(&mut self, ele: usize) {
        self.queue.push_back(ele);
    }
} */
