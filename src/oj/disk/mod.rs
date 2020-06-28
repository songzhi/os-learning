use std::cmp::Ordering;
use std::ops::Sub;

use intrusive_collections::{LinkedList, LinkedListLink};
use intrusive_collections::intrusive_adapter;
use itertools::Itertools;

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

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    /// to smaller end
    Left,
    /// to bigger end
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn is_left(&self) -> bool {
        matches!(self, Direction::Left)
    }
    fn is_right(&self) -> bool {
        matches!(self, Direction::Right)
    }
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
        let (sum, _) = self
            .requests
            .iter()
            .fold((0, curr), |(sum, curr), &destination| {
                (sum + distance(curr, destination), destination)
            });
        println!("fcfs: {}", sum);
    }
    fn sstf(&self, mut curr: usize, mut direction: Direction) {
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
        let mut sum = 0;
        while cursor.is_null() && cursor.peek_next().is_null() && cursor.peek_prev().is_null() {
            let left_gap: usize = cursor
                .get()
                .map(|request| curr - request.track)
                .unwrap_or(usize::MAX);
            let right_gap: usize = cursor
                .peek_next()
                .get()
                .map(|request| curr - request.track)
                .unwrap_or(usize::MAX);
            match left_gap.cmp(&right_gap) {
                Ordering::Less => direction = Direction::Left,
                Ordering::Equal => {}
                Ordering::Greater => direction = Direction::Right,
            }
            if direction.is_left() {
                curr = cursor.remove().unwrap().track;
                cursor.move_prev();
                sum += left_gap;
            } else {
                cursor.move_next();
                curr = cursor.remove().unwrap().track;
                cursor.move_prev();
                sum += right_gap;
            }
        }
        println!("sstf: {}", sum);
    }
    fn scan(&self, curr: usize, direction: Direction) {
        let mut min_track = usize::MAX;
        let mut max_track = usize::MIN;
        for &request in self.requests.iter() {
            if request < min_track {
                min_track = request;
            }
            if request > max_track {
                max_track = request;
            }
        }
        let sum = if curr <= min_track {
            max_track - curr
        } else if curr >= max_track {
            curr - min_track
        } else if direction.is_left() {
            (curr - min_track) + (max_track - min_track)
        } else {
            (max_track - curr) + (max_track - min_track)
        };
        println!("scan: {}", sum);
    }
    fn cscan(&self, curr: usize) {
        let mut min_track = usize::MAX;
        let mut max_track = usize::MIN;
        for &request in self.requests.iter() {
            if request < min_track {
                min_track = request;
            }
            if request > max_track {
                max_track = request;
            }
        }
        let sum = max_track - curr + max_track + min_track;
        println!("cscan: {}", sum);
    }
    fn nstep(&self, mut curr: usize, mut direction: Direction, gsize: usize) {
        fn scan(
            requests: impl Iterator<Item=usize>,
            curr: usize,
            direction: Direction,
        ) -> (usize, Direction, usize) {
            let mut min_track = usize::MAX;
            let mut max_track = usize::MIN;
            for request in requests {
                if request < min_track {
                    min_track = request;
                }
                if request > max_track {
                    max_track = request;
                }
            }
            let sum = if curr <= min_track {
                max_track - curr
            } else if curr >= max_track {
                curr - min_track
            } else if direction.is_left() {
                (curr - min_track) + (max_track - min_track)
            } else {
                (max_track - curr) + (max_track - min_track)
            };
            (curr, direction, sum)
        }
        let mut sum = 0;
        for requests in &self.requests.clone().into_iter().chunks(gsize) {
            let (curr_, direction_, sum_) = scan(requests, curr, direction);
            curr = curr_;
            direction = direction_;
            sum += sum_;
        }
        println!("nstep: {}", sum);
    }
}
