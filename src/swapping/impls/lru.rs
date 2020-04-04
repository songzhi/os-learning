use std::collections::HashMap;
use std::hash::Hash;

use crate::utils::deque::{Deque, Node, NonNullLink};

const LRU_DEFAULT_CAPACITY: usize = 128;

pub struct LruCache<T> {
    list: Deque<T>,
    hash: HashMap<T, NonNullLink<T>>,
    capacity: usize,
}

impl<T: Hash + Eq + Copy> Default for LruCache<T> {
    fn default() -> Self {
        Self {
            list: Deque::new(),
            hash: HashMap::new(),
            capacity: LRU_DEFAULT_CAPACITY,
        }
    }
}

impl<T: Hash + Eq + Copy> LruCache<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reserve(&mut self, capacity: usize) {
        self.capacity = capacity;
    }
    pub fn refer(&mut self, key: T) {
        if let Some(node) = self.hash.get(&key) {
            self.list.remove_node(node.clone());
            self.list.push_node_front(node.clone());
        } else if self.list.size() == self.capacity {
            if let Some(last) = self.list.pop_back() {
                self.hash.remove(&last);
            }
            let node = Node::new(key);
            self.hash.insert(key, node.clone());
            self.list.push_node_front(node);
        }
    }
}