//! Shortest Job First
use std::cmp::Reverse;

use keyed_priority_queue::KeyedPriorityQueue;

use crate::scheduling::{Os, PId, Scheduler};

/// Process which have the shortest burst time are scheduled first.
/// If two processes have the same bust time then FCFS is used to break the tie.
/// It is a non-preemptive scheduling algorithm.
#[derive(Default, Clone)]
pub struct ShortestJobFirstScheduler {
    ready_queue: KeyedPriorityQueue<PId, Reverse<u64>>,
}

impl ShortestJobFirstScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for ShortestJobFirstScheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: usize) {
        if let Some(process) = os.get_process(pid) {
            let burst_time = process.burst_time();
            self.ready_queue.push(pid, Reverse(burst_time));
        }
    }

    fn switch_process(&mut self, os: &mut Os) {
        os.switch_process(self.ready_queue.pop().map(|(pid, _)| pid));
    }

    fn desc(&self) -> &'static str {
        "Shortest Job First; Non-Preemptive; for Job"
    }

}
