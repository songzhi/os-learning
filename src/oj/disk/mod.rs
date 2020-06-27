use std::ops::Sub;

use intrusive_collections::{LinkedList, LinkedListLink};
use intrusive_collections::intrusive_adapter;

pub use parser::parse;

mod parser;

pub enum Command {
    /// First Command First Serve
    FCFS(usize),
    /// Shortest Seek Time First
    SSTF(usize, Direction),
    Scan(usize, Direction),
    CScan(usize),
    NStep(usize, Direction, usize),
}

pub enum Direction {
    /// to smaller end
    Left,
    /// to bigger end
    Right,
}

struct Request {
    link: LinkedListLink,
    track: usize,
}

impl Request {
    fn new(track: usize) -> Self {
        Self {
            link: Default::default(),
            track,
        }
    }
}

intrusive_adapter!(RequestAdapter = Box<Request>: Request { link: LinkedListLink});

pub struct Scheduler {
    tracks: usize,
    requests: Vec<usize>,
}

fn distance<T: Sub<Output=T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl Scheduler {
    pub fn new(tracks: usize, requests: Vec<usize>) -> Self {
        Self { tracks, requests }
    }
    pub fn run(&self, commands: Vec<Command>) {
        for command in commands {
            match command {
                Command::FCFS(curr) => self.fcfs(curr),
                Command::SSTF(curr, direction) => self.sstf(curr, direction),
                Command::Scan(curr, direction) => self.scan(curr, direction),
                Command::CScan(curr) => self.cscan(curr),
                Command::NStep(curr, direction, gsize) => self.nstep(curr, direction, gsize),
            }
        }
    }

    fn fcfs(&self, curr: usize) {
        let (count, _) = self
            .requests
            .iter()
            .fold((0, curr), |(count, curr), &destination| {
                (count + distance(curr, destination), destination)
            });
        println!("fcfs: {}", count);
    }
    fn sstf(&self, curr: usize, mut direction: Direction) {
        let mut sorted = self.requests.clone();
        sorted.sort();
        let mut requests = LinkedList::new(RequestAdapter::new());
        for t in sorted {
            requests.push_back(Box::new(Request::new(t)));
        }
        let mut cursor = requests.back_mut();
        while let Some(request) = cursor.get() {
            if request.track <= curr {
                break;
            }
            cursor.move_prev();
        }
        todo!()
    }
    fn scan(&self, curr: usize, direction: Direction) {
        todo!()
    }
    fn cscan(&self, curr: usize) {
        todo!()
    }
    fn nstep(&self, curr: usize, direction: Direction, gsize: usize) {
        todo!()
    }
}
