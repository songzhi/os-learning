//! Longest Remaining Job First
//!
use keyed_priority_queue::KeyedPriorityQueue;

use crate::scheduling::{Os, PId, Scheduler};

/// It is preemptive mode of LJF algorithm in which we give priority to the process having largest burst time remaining.
#[derive(Default, Clone)]
pub struct LongestRemainingJobFirstScheduler {
    ready_queue: KeyedPriorityQueue<PId, u64>,
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
        let current_remaining_time = os.get_process(pid).map(|p| p.remaining_time()).unwrap_or(0);
        if self
            .ready_queue
            .peek()
            .map_or(false, |(_, remaining_time)| remaining_time.gt(&current_remaining_time))
        {
            self.switch_process(os);
            self.ready_queue.push(pid, current_remaining_time);
        }
    }
}
