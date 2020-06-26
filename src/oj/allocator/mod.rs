use intrusive_collections::{LinkedList, LinkedListLink};
use intrusive_collections::intrusive_adapter;

pub use parser::parse;

mod parser;

pub enum Statement {
    Malloc(usize),
    MallocAddr(usize, usize),
    Free(usize),
    Compact,
}

#[derive(Clone, Debug)]
struct Area {
    link: LinkedListLink,
    start: usize,
    len: usize,
    handle: usize,
}

intrusive_adapter!(AreaAdapter = Box<Area>: Area {link: LinkedListLink});

impl Area {
    #[inline]
    fn start(&self) -> usize {
        self.start
    }
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
    #[inline]
    fn end(&self) -> usize {
        self.start + self.len
    }
    #[inline]
    fn is_intersected(&self, rhs: &Self) -> bool {
        !(self.start() >= rhs.end() || rhs.start() >= self.end())
    }
    #[inline]
    fn gap(&self, rhs: &Self) -> usize {
        if self.start() > rhs.end() {
            self.start() - rhs.end()
        } else if rhs.start() > self.end() {
            rhs.start() - self.end()
        } else {
            0
        }
    }
}

pub struct Allocator {
    size: usize,
    allocated: LinkedList<AreaAdapter>,
    next_handle: usize,
}

impl Allocator {
    pub fn run(mut self, statements: Vec<Statement>) {
        for stmt in statements {
            match stmt {
                Statement::Malloc(len) => self.malloc(len),
                Statement::MallocAddr(addr, len) => self.malloc_addr(addr, len),
                Statement::Free(handle) => self.free(handle),
                Statement::Compact => self.compact(),
            }
        }
    }
    fn malloc(&mut self, len: usize) {
        let mut cursor = self.allocated.back_mut();
        if cursor.get().is_none() {}
        while let Some(area) = cursor.get() {
            let area: &Area = area;
            let gap = cursor
                .peek_prev()
                .get()
                .map(|prev: &Area| prev.gap(area))
                .unwrap_or_else(|| area.start);
            if gap >= len {}
        }
    }
    fn malloc_addr(&mut self, addr: usize, len: usize) {}
    fn free(&mut self, handle: usize) {}
    fn compact(&mut self) {}
}
