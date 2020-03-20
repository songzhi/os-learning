//! Shortest Remaining Job First


use priority_queue::PriorityQueue;

use crate::scheduling::{Os, PId, Scheduler};

/// Process which have the shortest burst time are scheduled first.
/// If two processes have the same bust time then FCFS is used to break the tie.
/// It is a non-preemptive scheduling algorithm.
#[derive(Default, Clone)]
pub struct LongestRemainingJobFirstScheduler {
    ready_queue: PriorityQueue<PId, u64>,
}

impl LongestRemainingJobFirstScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for LongestRemainingJobFirstScheduler {
    fn on_process_ready(&mut self, os: &mut Os, pid: usize) {
        if let Some(process) = os.get_process(pid) {
            self.ready_queue.push(pid, process.remaining_time());
        }
    }

    fn switch_process(&mut self, os: &mut Os) {
        let pid = self.ready_queue.pop().map(|(pid, _)| pid);
        os.switch_process(pid);
    }

    fn desc(&self) -> &'static str {
        "Longest Remaining Job First"
    }

    fn on_process_burst(&mut self, os: &mut Os, pid: PId) {
        let remaining_time = os.get_process(pid).map(|p| p.remaining_time()).unwrap_or(0);
        self.ready_queue
            .change_priority(&pid, remaining_time);
    }
}
