use std::collections::VecDeque;

use crate::swapping::{Swapper, SWAPPER_DEFAULT_CAPACITY};

pub struct FifoSwapper<T> {
    deque: VecDeque<T>,
    capacity: usize,
}

impl<T> Default for FifoSwapper<T> {
    fn default() -> Self {
        Self {
            deque: VecDeque::with_capacity(SWAPPER_DEFAULT_CAPACITY),
            capacity: SWAPPER_DEFAULT_CAPACITY,
        }
    }
}

impl<T> FifoSwapper<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Eq> Swapper<T> for FifoSwapper<T> {
    fn reserve(&mut self, capacity: usize) {
        if let Some(additional) = capacity.checked_sub(self.deque.capacity()) {
            self.deque.reserve(additional);
        }
        self.capacity = capacity;
    }

    fn refer(&mut self, page: T) -> Result<(), Option<T>> {
        if self.deque.contains(&page) {
            Ok(())
        } else {
            let mut swapped_page = None;
            if self.deque.len() == self.capacity {
                swapped_page = self.deque.pop_front();
            }
            self.deque.push_back(page);
            Err(swapped_page)
        }
    }
}