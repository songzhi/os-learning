use std::collections::VecDeque;
use std::sync::Mutex;

use super::semaphore::Semaphore;

pub struct Queue<T> {
    queue: Mutex<VecDeque<T>>,
    empty_count: Semaphore,
    full_count: Semaphore,
}

unsafe impl<T: Send> Send for Queue<T> {}

unsafe impl<T: Sync> Sync for Queue<T> {}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(size)),
            empty_count: Semaphore::new(size),
            full_count: Semaphore::new(0),
        }
    }
    pub fn send(&self, t: T) {
        self.empty_count.acquire();
        self.queue
            .lock()
            .expect("failed to lock queue")
            .push_back(t);
        self.full_count.release();
    }
    pub fn recv(&self) -> T {
        self.full_count.acquire();
        let t = self
            .queue
            .lock()
            .expect("failed to lock queue")
            .pop_front()
            .unwrap();
        self.empty_count.release();
        t
    }
}
