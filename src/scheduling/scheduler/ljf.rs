//! Longest Job First
//!
use keyed_priority_queue::KeyedPriorityQueue;

use crate::scheduling::{Os, PId, Scheduler};

/// It is similar to SJF scheduling algorithm.
/// But, in this scheduling algorithm, we give priority to the process having the longest burst time.
/// This is non-preemptive in nature i.e., when any process starts executing,
/// can’t be interrupted before complete execution.
#[derive(Default, Clone)]
pub struct LongestJobFirstScheduler {
    ready_queue: KeyedPriorityQueue<PId, u64>,
}

impl LongestJobFirstScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for LongestJobFirstScheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: usize) {
        if let Some(process) = os.get_process(pid) {
            let burst_time = process.burst_time();
            self.ready_queue.push(pid, burst_time);
        }
    }

    fn switch_process(&mut self, os: &mut Os) {
        os.switch_process(self.ready_queue.pop().map(|(pid, _)| pid));
    }

    fn desc(&self) -> &'static str {
        "Longest Job First; Non-Preemptive; for Job"
    }

}
