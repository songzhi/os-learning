//! First Come First Serve
use std::collections::VecDeque;

use crate::scheduling::{Os, PId, Scheduler};

/// Simplest scheduling algorithm that schedules according to arrival times of processes.
/// First come first serve scheduling algorithm states that the process that requests the CPU first is allocated the CPU first.
/// It is implemented by using the FIFO queue. When a process enters the ready queue,
/// its PCB is linked onto the tail of the queue.
/// When the CPU is free, it is allocated to the process at the head of the queue.
/// The running process is then removed from the queue. FCFS is a non-preemptive scheduling algorithm.
#[derive(Default, Clone)]
pub struct FirstComeFirstServeScheduler {
    ready_queue: VecDeque<PId>,
}

impl FirstComeFirstServeScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for FirstComeFirstServeScheduler {
    fn on_process_ready(&mut self, _os: &mut Os, pid: usize) {
        self.ready_queue.push_back(pid);
    }
    fn switch_process(&mut self, os: &mut Os) {
        let pid = self.ready_queue.pop_front();
        os.switch_process(pid);
    }
    fn desc(&self) -> &'static str {
        "First Come First Serve"
    }
}
