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
    pub fn new(size: usize) -> Self {
        let mut allocated = LinkedList::new(AreaAdapter::new());
        let dummy = Box::new(Area {
            link: Default::default(),
            start: size,
            len: 0,
            handle: 0,
        });
        allocated.push_back(dummy);
        Self {
            size,
            allocated,
            next_handle: 1,
        }
    }
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
        if len > self.size || len == 0 {
            println!("0");
            return;
        }
        let mut cursor = self.allocated.back_mut();
        while let Some(area) = cursor.get() {
            let gap = cursor
                .peek_prev()
                .get()
                .map(|prev: &Area| prev.gap(area))
                .unwrap_or_else(|| area.start);
            if gap >= len {
                let new_area = Box::new(Area {
                    link: Default::default(),
                    start: area.start - len,
                    len,
                    handle: self.next_handle,
                });
                println!("{}: {}-{}", new_area.handle, new_area.start, new_area.len);
                self.next_handle += 1;
                cursor.insert_before(new_area);
                return;
            }
            cursor.move_prev();
        }
        println!("-1");
    }
    fn malloc_addr(&mut self, addr: usize, len: usize) {
        if len > self.size || len == 0 || addr >= self.size || addr + len > self.size {
            println!("0");
            return;
        }
        let new_area = Area {
            link: Default::default(),
            start: addr,
            len,
            handle: self.next_handle,
        };
        let end = addr + len;
        let mut cursor = self.allocated.back_mut();
        while let Some(area) = cursor.get() {
            if area.start() <= end {
                break;
            }
            cursor.move_prev();
        }
        let left: Option<&Area> = cursor.get();
        let right: Option<&Area> = cursor.peek_next().get();
        let can_allocate = left
            .map(|left| !left.is_intersected(&new_area))
            .unwrap_or(true)
            && right
            .map(|right| !right.is_intersected(&new_area))
            .unwrap_or(true);
        if can_allocate {
            self.next_handle += 1;
            println!("{}: {}-{}", new_area.handle, new_area.start, new_area.len);
            cursor.insert_before(Box::new(new_area));
        }
    }
    fn free(&mut self, handle: usize) {
        let mut cursor = self.allocated.back_mut();
        while let Some(area) = cursor.get() {
            if area.handle == handle {
                cursor.remove();
                println!("1");
                return;
            }
            cursor.move_prev();
        }
        println!("0");
    }
    fn compact(&mut self) {
        let mut addr = self.size;
        let mut cursor = self.allocated.cursor_mut();
        while let Some(area) = cursor.get() {
            addr -= area.len();
            let updated_area = Area {
                link: Default::default(),
                start: addr,
                len: area.len(),
                handle: area.handle,
            };
            cursor.replace_with(Box::new(updated_area)).unwrap();
            cursor.move_prev();
        }
    }
}
