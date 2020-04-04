use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}


type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type NonNullLink<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links,everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list,need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                // total: +2 new_head, +0 old_head -- OK!
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head) // +1 new_head
                // total: +2 new_head -- OK
            }
        }
        self.size += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // not emptying list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    // total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take(); // -1 old
                    // total: -2 old
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn remove_node(&mut self, node: NonNullLink<T>) {
        let mut node = node.borrow_mut();
        match (node.prev.take(), node.next.take()) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next.replace(next.clone());
                next.borrow_mut().prev.replace(prev);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next.take();
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                next.borrow_mut().prev.take();
                self.head.replace(next);
            }
            (None, None) => unreachable!()
        }
        self.size -= 1;
    }

    pub fn push_node_front(&mut self, node: NonNullLink<T>) {
        match self.head.take() {
            None => {
                let mut new_head = node.borrow_mut();
                new_head.next.take();
                new_head.prev.take();
                self.head.replace(node.clone());
                self.tail.replace(node.clone());
            }
            Some(old_head) => {
                old_head.borrow_mut().prev.replace(node.clone());
                let mut new_head = node.borrow_mut();
                new_head.prev.take();
                new_head.next.replace(old_head);
                self.head.replace(node.clone());
            }
        }
        self.size += 1;
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(Deque<T>);

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::{Deque, Node};

    #[test]
    fn basics() {
        let mut list = Deque::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = Deque::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = Deque::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_and_push_node() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let mut list = Deque::new();
        list.push_node_front(node1.clone());
        list.push_node_front(node2);
        assert_eq!(&*list.peek_front().unwrap(), &2);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        list.remove_node(node1.clone());
        assert_eq!(&*list.peek_front().unwrap(), &2);
        assert_eq!(&*list.peek_back().unwrap(), &2);
        list.push_node_front(node1);
        assert_eq!(&*list.peek_front().unwrap(), &1);
    }
}
