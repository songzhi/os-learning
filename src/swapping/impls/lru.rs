use std::collections::HashMap;
use std::hash::Hash;

use crate::swapping::{Swapper, SWAPPER_DEFAULT_CAPACITY};
use crate::utils::deque::{Deque, Node, NonNullLink};

pub struct LruSwapper<T> {
    list: Deque<T>,
    hash: HashMap<T, NonNullLink<T>>,
    capacity: usize,
}

impl<T: Hash + Eq + Copy> Default for LruSwapper<T> {
    fn default() -> Self {
        Self {
            list: Deque::new(),
            hash: HashMap::new(),
            capacity: SWAPPER_DEFAULT_CAPACITY,
        }
    }
}

impl<T: Hash + Eq + Copy> LruSwapper<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Hash + Eq + Copy> Swapper<T> for LruSwapper<T> {
    fn reserve(&mut self, capacity: usize) {
        self.capacity = capacity;
    }
    fn refer(&mut self, page: T) -> Result<(), Option<T>> {
        if let Some(node) = self.hash.get(&page) {
            self.list.remove_node(node.clone());
            self.list.push_node_front(node.clone());
            Ok(())
        } else {
            let mut swapped_page = None;
            if self.list.size() == self.capacity {
                if let Some(last) = self.list.pop_back() {
                    self.hash.remove(&last);
                    swapped_page = Some(last);
                }
            }
            let node = Node::new(page);
            self.hash.insert(page, node.clone());
            self.list.push_node_front(node);
            Err(swapped_page)
        }
    }
}