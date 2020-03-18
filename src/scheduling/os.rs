use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use indexmap::IndexMap;
use pendulum::{HashedWheel, HashedWheelBuilder, Pendulum};

use super::process::{PId, Process};
use super::scheduler::Scheduler;
use super::TICK;

pub struct Os {
    pub(crate) clock: u64,
    processes: IndexMap<PId, Process>,
    pub(crate) waiting: HashedWheel<PId>,
    running_process_pid: Option<PId>,
    scheduler: Rc<RefCell<dyn Scheduler>>,
    completed_process_count: usize,
}

impl Os {
    pub fn new(processes: IndexMap<PId, Process>, scheduler: Rc<RefCell<dyn Scheduler>>) -> Self {
        let mut waiting = HashedWheelBuilder::default()
            .with_tick_duration(Duration::from_millis(10))
            .build();
        for p in processes.values() {
            waiting.insert_timeout(Duration::from_millis(p.arrival_time()), p.id);
        }
        Self {
            clock: 0,
            processes,
            waiting,
            running_process_pid: None,
            scheduler,
            completed_process_count: 0,
        }
    }
    pub fn run(&mut self) {
        while !self.is_completed() {
            self.tick();
        }
    }
    pub fn tick(&mut self) {
        self.clock += TICK;
        self.waiting.tick();
        let scheduler = self.scheduler.clone();
        let mut scheduler = scheduler.borrow_mut();
        scheduler.on_tick(self);
    }

    pub fn running_process(&mut self) -> Option<&mut Process> {
        self.running_process_pid
            .and_then(move |pid| self.processes.get_mut(&pid))
    }
    pub fn get_process(&self, pid: PId) -> Option<&Process> {
        self.processes.get(&pid)
    }
    pub fn get_mut_process(&mut self, pid: PId) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }
    pub fn await_process(&mut self, pid: PId, timeout: u64) {
        self.waiting
            .insert_timeout(Duration::from_millis(timeout), pid);
    }
    pub fn complete_process(&mut self, pid: PId) {
        self.completed_process_count += 1;
    }
    pub fn is_completed(&self) -> bool {
        self.completed_process_count == self.processes.len()
    }
}
